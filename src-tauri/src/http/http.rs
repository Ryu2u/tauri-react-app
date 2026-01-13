use crate::sqlite::sqlite::sqlite::{delete_token, get_token};
use crate::sqlite::structs::{HttpError, HttpResult};
use crate::{back_to_login, SqliteRbatis, WsConnectFlag};
use log::{error, info};
use reqwest::{header, Error, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use tauri::{AppHandle, Manager, State, Wry};

const AUTH_HEADER: &str = "Authorization";
const TOKEN_BEARER: &str = "Bearer ";

/// Http Get 请求接口 返回json格式
pub async fn http_get<T: DeserializeOwned>(
    path: String,
    state: State<'_, SqliteRbatis>,
    app_handle: AppHandle<Wry>,
) -> Result<HttpResult<T>, HttpError> {
    let state = state.db.lock().await;
    let token = get_token(&*state).await;

    let client = reqwest::Client::new();
    let url = format!("http://localhost:9090{}", path);
    let res = client
        .get(url)
        .header(AUTH_HEADER, format!("{}{}", TOKEN_BEARER, token))
        .send()
        .await;

    handle_response(res, app_handle).await
}

/// Http Post 请求接口 返回json格式
pub async fn http_post<T: DeserializeOwned, E: Serialize + ?Sized>(
    path: String,
    state: State<'_, SqliteRbatis>,
    json: &E,
    app_handle: AppHandle<Wry>,
) -> Result<HttpResult<T>, HttpError> {
    let state = state.db.lock().await;
    let token = get_token(&*state).await;
    let client = reqwest::Client::new();
    let host = env::var("HTTP_URL").expect("env file don't exists HTTP_URL");
    let url = format!("{}{}", host, path);
    let res = client
        .post(url)
        .header(AUTH_HEADER, format!("{}{}", TOKEN_BEARER, token))
        .json(json)
        .send()
        .await;

    handle_response(res, app_handle).await
}

pub async fn http_post_no_auth<T: DeserializeOwned, E: Serialize + ?Sized>(
    path: String,
    json: &E,
    app_handle: AppHandle<Wry>,
) -> Result<HttpResult<T>, HttpError> {
    let client = reqwest::Client::new();
    let host = env::var("HTTP_URL").expect("env file don't exists HTTP_URL");
    let url = format!("{}{}", host, path);
    let res = client
        .post(url)
        .header(header::ACCEPT, "application/json")
        .header(header::CONTENT_TYPE, "application/json")
        .json(json)
        .send()
        .await;
    handle_response(res, app_handle).await
}

/// 处理http 响应
pub async fn handle_response<T: DeserializeOwned>(
    res: Result<Response, Error>,
    app_handle: AppHandle<Wry>,
) -> Result<HttpResult<T>, HttpError> {
    match res {
        Ok(response) => {
            let json = response.json::<HttpResult<T>>().await;
            match json {
                Ok(data) => {
                    info!("http code : {:?}", data.code);
                    if data.code == 403 || data.code == 401 {
                        let flag_state: State<'_, WsConnectFlag> = app_handle.state();
                        let lock = flag_state.connected.lock().await;
                        if let None = app_handle.get_window("login") {
                            let app_clone = app_handle.clone();
                            let state: State<'_, SqliteRbatis> = app_clone.try_state().unwrap();
                            delete_token(state).await;
                            back_to_login(app_clone);
                            let login_window = app_handle.get_window("login").unwrap();
                            tauri::api::dialog::confirm(
                                Some(&login_window),
                                "Tauri",
                                "令牌已过期，请重新登录!",
                                move |answer| {
                                    if answer {
                                        tauri::async_runtime::block_on(async move {});
                                    }
                                },
                            );
                        }
                        match *lock {
                            _ => {
                                info!("释放锁!");
                            }
                        }
                        drop(lock);
                    }
                    Ok(data)
                }
                Err(e) => {
                    error!("反序列化失败! -> {:?}", e);
                    Err(HttpError::CustomError("Error".to_string()))
                }
            }
        }
        Err(_e) => {
            error!("读取响应失败!");
            Err(HttpError::CustomError("http error".to_string()))
        }
    }
}
