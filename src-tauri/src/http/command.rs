use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use rbatis::RBatis;
use rbdc_sqlite::SqliteDriver;
use tauri::{AppHandle, Manager, State, Wry};
use crate::command::route_to_admin;
use crate::sqlite::{AuthHeader, ChatMessage, ChatRoom, HttpError, HttpResult, User};
use crate::sqlite::sqlite::sqlite::get_token;
use crate::http::secure::{decode_msg, get_public_key, encode_msg};
use crate::http::{http_get, http_post_no_auth, http_post};


#[tauri::command]
pub async fn get_room_info(room_id: String, state: State<'_, RBatis>, app_handle:
AppHandle<Wry>)
                           -> Result<HttpResult<ChatRoom>, HttpError> {
    match http_get::<ChatRoom>(format!("/chat-room/{}", room_id), state, app_handle).await {
        Ok(res) => {
            println!("{:?}", res);
            Ok(res)
        }
        Err(e) => {
            println!("调用失败 -> /chat-room/{}", room_id);
            Err(e)
        }
    }
}


#[tauri::command]
pub async fn check_login(user_id: i32, app_handle: AppHandle<Wry>) -> Result<(),
    HttpError> {
    let mut sqlite_url = env::var("SQLITE_URL").unwrap();
    sqlite_url = format!("{}{}.db", sqlite_url, user_id);
    let rb = RBatis::new();
    rb.init(SqliteDriver {}, sqlite_url.as_str()).unwrap();
    let rb_copy = rb.clone();
    let token = get_token(&rb_copy).await;
    if token != "" {
        app_handle.manage(rb);
        if let None = app_handle.get_window("main") {
            println!("main 不存在!");
            route_to_admin(app_handle).await;
        } else {
            println!("main 已存在!");
        }
    }
    Ok(())
}


#[tauri::command]
pub async fn room_msg_list(room_id: String, send_time: String,
                           state: State<'_, RBatis>,
                           app_handle: AppHandle<Wry>) -> Result<HttpResult<Vec<ChatMessage>>,
    HttpError> {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("roomId".to_string(), room_id.to_string());
    map.insert("sendTime".to_string(), send_time.to_string());

    match http_post("/chat-message/list".to_string(), state, &map, app_handle).await {
        Ok(res) => {
            println!("{:?}", res);
            Ok(res)
        }
        Err(e) => {
            println!("调用失败 -> /chat-message/list");
            Err(e)
        }
    }
}


#[tauri::command]
pub async fn get_chat_room_list(state: State<'_, RBatis>, app_handle: AppHandle<Wry>) ->
Result<HttpResult<Vec<ChatRoom>>, HttpError> {
    match http_get::<Vec<ChatRoom>>("/chat-room/all".to_string(), state, app_handle).await {
        Ok(res) => {
            println!("{:?}", res);
            Ok(res)
        }
        Err(e) => {
            println!("调用失败 -> /chat-room/all");
            Err(e)
        }
    }
}


/// 登录接口
#[tauri::command]
pub async fn login(username: &str, password: &str, remember_me: bool, app_handle: AppHandle<Wry>) ->
Result<HttpResult<String>, HttpError> {
    let username = username.as_bytes();
    let password = password.as_bytes();

    let username = encode_msg(username);
    let password = encode_msg(password);

    let public_key = get_public_key();
    println!("{}", public_key);

    let mut map = HashMap::new();
    map.insert("username", username);
    map.insert("password", password);
    map.insert("rememberMe", remember_me.to_string());
    map.insert("publicKey", public_key);

    let res = http_post_no_auth::<AuthHeader, HashMap<&str, String>>("/login".to_string(),
                                                                     &map, app_handle.clone()).await;
    match res {
        Ok(result) => {
            let auth = result.data.clone();
            if let Some(mut data) = auth {
                let key = data.key.clone();
                println!("data key : {}", key);
                let decode_key = decode_msg(&key);
                println!("decode key : {}", decode_key);
                data.key = decode_key;
                let key = data.key.clone();
                println!("{:?}", data);
                let table = data.clone();

                // connect to database
                let mut sqlite_url = env::var("SQLITE_URL").unwrap();
                sqlite_url = format!("{}{}.db", sqlite_url, data.key);

                let rb = RBatis::new();
                rb.init(SqliteDriver {}, sqlite_url.as_str()).unwrap();

                // 执行初始化数据库sql
                let mut sql_file = File::open("main.sql").unwrap();
                let mut sql = String::new();
                sql_file.read_to_string(&mut sql).expect("read sql failed");

                rb.exec(sql.as_str(), vec![]).await.expect("exec sql failed");

                let insert_data = AuthHeader::insert(&rb, &table).await;
                println!("insert_data : {:?}", insert_data);
                app_handle.manage(data);
                app_handle.manage(rb);
                let result = HttpResult {
                    code: result.code,
                    msg: result.msg,
                    data: Some(key),
                };
                Ok(result)
            } else {
                let result = HttpResult {
                    code: result.code,
                    msg: result.msg,
                    data: None,
                };
                Ok(result)
            }
        }
        Err(e) => {
            println!("Http Err");
            if let HttpError::RequestError(status) = e {
                let code = status.as_u16();
                println!("{}", code);
                let result = HttpResult {
                    code: code as i32,
                    msg: "".to_string(),
                    data: None,
                };
                Ok(result)
            } else {
                Err(e)
            }
        }
    }
}


/// 用户当前登录用户具体信息
#[tauri::command]
pub async fn get_user_info(state: State<'_, RBatis>, app_handle: AppHandle<Wry>) ->
Result<HttpResult<User>, HttpError> {
    match http_get::<User>("/user/info".to_string(), state, app_handle.clone()).await {
        Ok(res) => {
            println!("{:?}", res);
            let user = res.data.clone().unwrap();
            app_handle.manage(user);
            Ok(res)
        }
        Err(e) => {
            println!("Error get_user_info");
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn get_sys_time(state: State<'_, RBatis>, app_handle: AppHandle<Wry>) -> Result<HttpResult<i64>, HttpError> {
    match http_get::<i64>("/chat-message/time".to_string(), state, app_handle.clone()).await {
        Ok(res) => {
            println!("{:?}", res);
            Ok(res)
        }
        Err(e) => {
            println!("Error get_sys_time");
            Err(e)
        }
    }
}

