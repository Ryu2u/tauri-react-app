pub mod ws {
  use futures_util::SinkExt;
  use tauri::{AppHandle, Event, Manager, Wry};
  use tokio::task::block_in_place;
  use tokio_tungstenite::{
    tungstenite::Result,
  };
  use entity::chat_message_pack::Obj;
  use entity::{ChatMessagePack, LoginMessage, MsgType, ProstMessage};
  use crate::system_tray_flicker;

  #[tauri::command]
  pub async fn connect_websocket(app_handle: AppHandle<Wry>) -> String {
    // if let Ok(_) = connect_websocket(&app_handle).await {
    //   format!("Ok")
    // } else {
    //   format!("Err")
    // }
    connect_ws_async(&app_handle).await.unwrap();
    format!("Ok")
  }

  async fn connect_ws_async(app_handle: &AppHandle<Wry>) -> Result<()> {
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

    let ( ws_stream, _) = match connect_response {
      Ok(ws, ..) => {
        ws
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
    println!("WebSocket handshake has been successfully completed");

    // 将websocket分割为 写 和 读，可以单独分割使用
    let (write, read) = ws_stream.split();

    // 需要将写包装为Arc，使其可以在线程中传递
    let mutex_write = Arc::new(Mutex::new(write));
    let mutex_read = Mutex::new(read);

    let handle_write = app_handle.clone();
    let handle_read = app_handle.clone();

    // 注册监听前端的消息发送事件，当前端触发事件时调用websocket写，发送消息至服务器
    let _event = handle_write.listen_global("msg_send", move |event: Event| {
      let msg = event.payload().unwrap();
      let mutex_write = mutex_write.clone();
      block_in_place(move || {
        let mutex_write = mutex_write.clone();
        tauri::async_runtime::block_on(async move {
          println!("Send : {}", msg);
          let obj = Obj::LoginMessage(LoginMessage {
            user_id: 1,
            username: "admin".to_string(),
          });
          let pack = ChatMessagePack::new("123", MsgType::LoginMessageType, Some(obj));
          let len = ProstMessage::encoded_len(&pack);
          let mut buf: Vec<u8> = vec![];
          buf.reserve(len);
          pack.encode(&mut buf).unwrap();
          println!("{:?}", buf);
          // 发送消息
          mutex_write.lock().await.send(Message::binary(buf)).await.unwrap();
        });
      });
    });

    // 异步任务，循环读
    tokio::spawn(async move {
      let mut read_data = mutex_read.lock().await;
      while let Some(res_msg) = read_data.next().await {
        if let Ok(msg) = res_msg {
          if msg.is_text() {
            println!("GOT  TEXT : {}", msg);
            system_tray_flicker(&handle_read);
            handle_read.emit_all("msg_read", msg.into_text().unwrap()).expect("read msg failed");
          } else if msg.is_binary() {
            let data = msg.into_data();
            println!("GOT  BINARY : {:?}", data);
            let obj: ChatMessagePack = entity::ProstMessage::decode(&*data).unwrap();
            println!("{:?}", obj);
            handle_read.emit_all("msg_read", obj).expect("read msg failed");
          } else if msg.is_close() {
            println!("websocket is closed");
          }
        } else {
          println!("websocket is closed");
        }
      }
    });


    Ok(())
  }
}