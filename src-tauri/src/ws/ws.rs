use std::any::Any;
use std::fmt::format;
use std::ops::Deref;
use std::sync::Arc;
use futures_util::SinkExt;
use futures_util::stream::{SplitSink, SplitStream};
use rbatis::RBatis;
use rbatis::rbdc::Uuid;
use tauri::{App, AppHandle, Event, Manager, State, Wry};
use tauri::async_runtime::handle;
use tokio::net::TcpStream;
use tokio::task::block_in_place;
use tokio_tungstenite::{MaybeTlsStream, tungstenite::Result, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use entity::chat_message_pack::Obj;
use entity::{ChatMessagePack, GroupMessage, LoginMessage, MsgType, ProstMessage};
use crate::{ConnectedEnum, system_tray_flicker, WsConnectFlag};
use crate::sqlite::sqlite::sqlite::get_token;
use crate::sqlite::{ChatMessage, User};

/// WebSocket连接
#[tauri::command]
pub async fn connect_websocket(app_handle: AppHandle<Wry>, state: State<'_, WsConnectFlag>)
                               -> tauri::Result<()> {
    connect_ws_async(&app_handle, state).await.unwrap();
    Ok(())
}


/// 连接websocket 主要函数
async fn connect_ws_async(app_handle: &AppHandle<Wry>, state: State<'_, WsConnectFlag>) ->
Result<()> {
    use url::Url;
    use futures_util::{StreamExt};
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
    use tokio::sync::Mutex;
    use std::env;
    use std::sync::Arc;

    let ws_url = env::var("WS_URL").expect("can not find .env -> WS_URL");
    let url = Url::parse(&ws_url).expect("bad url!");

    // 连接websocket服务器
    let connect_response = connect_async(url).await;

    let (ws_stream, _) = match connect_response {
        Ok(ws, ..) => {
            let mut guard = state.connected.lock().await;
            match *guard {
                ConnectedEnum::YES => {
                    println!("WebSocket is connected");
                    return Ok(());
                }
                ConnectedEnum::NO => {
                    println!("WebSocket正在连接...");
                    *guard = ConnectedEnum::YES;
                    ws
                }
            }
        }
        Err(_) => {
            let windows = app_handle.windows();
            for key in windows.keys() {
                if let Some(window) = windows.get(key) {
                    tauri::api::dialog::confirm(Some(&window), "Error", "无法连接Websocket服务器!", move
                        |_answer| {
                        panic!("can't connect websocket server!");
                    });
                }
            }
            panic!("can't connect websocket server!");
        }
    };

    println!("WebSocket connect success");

    // 将websocket分割为 写 和 读，可以单独分割使用
    let (write, read) = ws_stream.split();

    // 需要将写包装为Arc，使其可以在线程中传递
    let mutex_write = Arc::new(Mutex::new(write));
    let mutex_read = Mutex::new(read);

    // let handle_read = app_handle.clone();
    let handle = app_handle.clone();

    // 注册监听前端的消息发送事件，当前端触发事件时调用websocket写，发送消息至服务器
    listen_group_msg(handle, app_handle.clone(), mutex_write.clone());

    handle_ws_read(app_handle.clone(), mutex_read, state);

    Ok(())
}


type WebSocketWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WebSocketReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

fn listen_group_msg(handle: AppHandle<Wry>, handle_write: AppHandle<Wry>,
                    mutex_write: Arc<tokio::sync::Mutex<WebSocketWriter>>) {
    // 注册监听前端的消息发送事件，当前端触发事件时调用websocket写，发送消息至服务器
    let _event = handle_write.listen_global("group_msg_send", move |event: Event| {
        println!("GOT Group msg payload ==> ");
        let chat_msg_json = event.payload().unwrap();
        println!("{:?}", chat_msg_json);
        let mut msg: ChatMessage = serde_json::from_str(chat_msg_json).expect("can't \
        resolve ChatMessage");
        println!("{:?}", msg);

        let mutex_write = mutex_write.clone();
        let handle = handle.clone();
        block_in_place(move || {
            let mutex_write = mutex_write.clone();
            tauri::async_runtime::block_on(async move {
                let user_state: State<'_, User> = handle.try_state().unwrap();
                println!("Send : {}", chat_msg_json);
                println!("{:?}", *user_state);

                let mut chat_message = entity::message::ChatMessage::new();
                chat_message.id = msg.id.take().expect("id can't take");
                chat_message.chat_room_id = msg.roomId.take().expect("roomId can't take");
                chat_message.content = msg.content.take().expect("content can't take");
                chat_message.sender_id = user_state.id;

                let obj = Obj::GroupMessage(GroupMessage {
                    group_id: chat_message.chat_room_id.clone(),
                    group_name: "".to_string(),
                    chat_message: Some(chat_message),
                });
                send_ws_message(mutex_write, handle, MsgType::GroupMessageType, obj).await;
            });
        });
    });
}

/// 处理WebSocket 读事件
fn handle_ws_read(handle_read: AppHandle<Wry>,
                  mutex_read: tokio::sync::Mutex<WebSocketReader>,
                  state: State<'_, WsConnectFlag>) {
    use futures_util::{StreamExt};
    let guard = state.connected.clone();
    // 异步任务，循环读
    tokio::spawn(async move {
        let mut read_data = mutex_read.lock().await;
        while let Some(res_msg) = read_data.next().await {
            if let Ok(msg) = res_msg {
                system_tray_flicker(&handle_read);
                if msg.is_text() {
                    println!("GOT  TEXT : {}", msg);
                    handle_read.emit_all("msg_read", msg.into_text().unwrap()).expect("read msg failed");
                } else if msg.is_binary() {
                    let data = msg.into_data();
                    let obj: ChatMessagePack = entity::ProstMessage::decode(&*data).unwrap();
                    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>");
                    println!("GOT  BINARY: {:?}", obj);
                    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>");
                    // todo! 根据不同的消息类型返回不同的事件

                    let obj_data = obj.obj.unwrap();
                    match obj_data {
                        Obj::GroupMessage(msg) => {
                            let chat_msg = msg.chat_message.unwrap();
                            println!("{:?}", chat_msg);
                            // let room_id = chat_msg.chat_room_id.clone();
                            handle_read.emit_all("msg_read", chat_msg).expect
                            ("read msg failed");
                        }
                        Obj::AckMessage(ack) => {
                            handle_read.emit_all("msg_ack", ack).expect("can't emit msg_ack");
                        }
                        _ => {}
                    }
                }
            } else {
                {
                    let mut flag = guard.lock().await;
                    *flag = ConnectedEnum::NO;
                } // 需要在这里释放锁，不然会造成死锁,当然下面也可以直接break
                panic!("websocket is closed");
            }
        }
    });
}


async fn send_ws_message(mutex_write: Arc<tokio::sync::Mutex<WebSocketWriter>>,
                         handle: AppHandle<Wry>,
                         msg_type: MsgType,
                         obj: Obj) {
    let rb_state: State<'_, RBatis> = handle.try_state().unwrap();
    let token = get_token(&rb_state).await;
    let pack = ChatMessagePack::new(token.as_str(), msg_type, Some(obj));
    let len = ProstMessage::encoded_len(&pack);
    let mut buf: Vec<u8> = vec![];
    buf.reserve(len);
    pack.encode(&mut buf).unwrap();
    println!("{:?}", buf);
    // 发送消息
    if let Ok(_) = mutex_write.lock().await.send(Message::binary(buf)).await {
        println!("发送成功!");
    } else {
        println!("发送失败!");
    }
}


