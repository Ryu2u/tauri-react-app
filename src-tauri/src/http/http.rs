pub mod http {
    use std::collections::HashMap;
    use std::env;
    use std::fmt::{Debug, format};
    use std::fs::File;
    use std::io::Read;
    use base64::Engine;
    use reqwest::{Error, header, Response};
    use rsa::{RsaPrivateKey};
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use tauri::{App, AppHandle, Manager, State, Wry};
    use crate::back_to_login;
    use crate::http::structs::structs::{AuthHeader, HttpResult};
    use crate::http::{ChatRoom, HttpError, User};

    const AUTH_HEADER: &str = "Authorization";
    const TOKEN_BEARER: &str = "Bearer ";

    /// 用户当前登录用户具体信息
    #[tauri::command]
    pub async fn get_user_info(state: State<'_, AuthHeader>, app_handle: AppHandle<Wry>) ->
    Result<HttpResult<User>, HttpError> {
        let token = state.Authorization.clone();
        println!("token : {}", token);

        match http_get::<User>("/user/info".to_string(), state, app_handle).await {
            Ok(res) => {
                println!("{:?}", res);
                Ok(res)
            }
            Err(e) => {
                println!("Errrrrr");
                Err(e)
            }
        }
    }

    /// 登录接口
    #[tauri::command]
    pub async fn login(username: &str, password: &str, app_handle: AppHandle<Wry>) -> Result<HttpResult<()>, HttpError> {
        let username = username.as_bytes();
        let password = password.as_bytes();

        let username = encode_msg(username);
        let password = encode_msg(password);

        let public_key = get_public_key();
        println!("{}", public_key);

        let mut map = HashMap::new();
        map.insert("username", username);
        map.insert("password", password);
        map.insert("publicKey", public_key);

        let res = http_post_no_auth::<AuthHeader, HashMap<&str, String>>("/login".to_string(),
                                                                         &map, app_handle.clone())
            .await;
        match res {
            Ok(result) => {
                let auth = result.data.clone();
                println!("{:?}", auth);
                if let Some(mut data) = auth {
                    app_handle.manage(data);
                }
                let result = HttpResult {
                    code: result.code,
                    msg: result.msg,
                    data: None,
                };
                Ok(result)
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

    /// 获取公钥
    pub fn get_public_key() -> String {
        use rsa::{RsaPublicKey};
        use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
        use base64::{engine::general_purpose};

        let mut file: File = std::fs::File::open("public_key.txt").unwrap();
        let mut public_key = String::new();
        file.read_to_string(&mut public_key).expect("can't read file");
        let local_public_key = RsaPublicKey::from_public_key_pem(&public_key).unwrap();
        let doc = local_public_key.to_public_key_der().unwrap();

        let base64_encode = general_purpose::STANDARD_NO_PAD.encode(doc.to_vec());

        base64_encode
    }

    /// 使用后端公钥加密需要发送给后端的内容
    /// 返回的String 为base64 加密后的字符串
    pub fn encode_msg(raw_str: &[u8]) -> String {
        use base64::engine::general_purpose;
        use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
        use rsa::pkcs8::{DecodePublicKey};
        let server_public_key = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC3ktCxwURY+Pkz49sDbmy2/WWv
j6X3noeoh0coEY41DO5meYIAebkIqiYR2Hkhkf6s0SIdZT1gmZQQx2ZPmb/bI4L2
CE0ILa/ZabzIHgcBPdouzuj/whV/WhKx0y5uACsaEg+Khr8rmBbh5EGyw4EUWnA1
4/pUds5rdAwpfZiM9wIDAQAB
-----END PUBLIC KEY-----";
        let server_public_key = RsaPublicKey::from_public_key_pem(&server_public_key).unwrap();
        let mut rng = rand::thread_rng();
        let encode_str = server_public_key.encrypt(&mut rng, Pkcs1v15Encrypt, raw_str)
            .unwrap();
        let base64_encode = general_purpose::STANDARD_NO_PAD.encode(&encode_str[..]);
        println!("{}", base64_encode);
        base64_encode
    }

    /// 使用本地私钥解密后端加密的内容
    /// msg 为base64 加密后的字符串
    pub fn decode_msg(msg: &str) -> String {
        use base64::engine::general_purpose;
        use rsa::{Pkcs1v15Encrypt};
        use rsa::pkcs8::{DecodePrivateKey};

        let mut file: File = std::fs::File::open("private_key.txt").unwrap();
        let mut private_key = String::new();
        file.read_to_string(&mut private_key).expect("can't read file");
        let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key).unwrap();

        let vec = general_purpose::STANDARD.decode(msg.to_string()).unwrap();

        let decode_str = private_key.decrypt(Pkcs1v15Encrypt, &vec).unwrap();
        let res = String::from_utf8(decode_str);
        println!("{:?}", res);
        match res {
            Ok(str) => {
                str
            }
            _ => {
                "".to_string()
            }
        }
    }

    /// Http Get 请求接口 返回json格式
    async fn http_get<T: DeserializeOwned>(path: String, state: State<'_, AuthHeader>,
                                           app_handle: AppHandle<Wry>)
                                           ->
                                           Result<HttpResult<T>, HttpError> {
        let token = state.Authorization.clone();
        let client = reqwest::Client::new();
        let url = format!("http://localhost:9090{}", path);
        let res = client.get(url)
            .header(AUTH_HEADER, format!("{}{}", TOKEN_BEARER, token))
            .send().await;

        handle_response(res, app_handle).await
    }


    /// Http Post 请求接口 返回json格式
    async fn http_post<T: DeserializeOwned, E: Serialize + ?Sized>(path: String, state: State<'_,
        AuthHeader>, json: &E, app_handle: AppHandle<Wry>) ->
                                                                   Result<HttpResult<T>, HttpError> {
        let token = state.Authorization.clone();
        let client = reqwest::Client::new();
        let host = env::var("HTTP_URL").expect("env file don't exists HTTP_URL");
        let url = format!("{}{}", host, path);
        let res = client.post(url)
            .header(AUTH_HEADER, format!("{}{}", TOKEN_BEARER, token))
            .json(json)
            .send().await;

        handle_response(res, app_handle).await
    }

    async fn http_post_no_auth<T: DeserializeOwned, E: Serialize + ?Sized>(path: String, json: &E, app_handle: AppHandle<Wry>) ->
    Result<HttpResult<T>, HttpError> {
        let client = reqwest::Client::new();
        let host = env::var("HTTP_URL").expect("env file don't exists HTTP_URL");
        let url = format!("{}{}", host, path);
        let res = client.post(url)
            .header(header::ACCEPT, "application/json")
            .header(header::CONTENT_TYPE, "application/json")
            .json(json)
            .send().await;
        handle_response(res, app_handle).await
    }


    async fn handle_response<T: DeserializeOwned>(res: Result<Response, Error>, app_handle: AppHandle<Wry>) -> Result<HttpResult<T>, HttpError> {
        match res {
            Ok(response) => {
                let json = response.json::<HttpResult<T>>().await;
                match json {
                    Ok(data) => {
                        println!("http code : {:?}", data.code);
                        if data.code == 403 {
                            let windows = app_handle.clone().windows();
                            for key in windows.keys() {
                                let app_clone = app_handle.clone();
                                let window_opt = windows.get(key);
                                if let Some(window) = window_opt {
                                    tauri::api::dialog::confirm(Some(&window), "Tauri", "令牌已过期，请重新登录!",
                                                                move
                                                                    |answer| {
                                                                    back_to_login(app_clone);
                                                                });
                                }
                            }
                        }
                        Ok(data)
                    }
                    Err(e) => {
                        println!("反序列化失败! -> {:?}",e);
                        Err(HttpError::CustomError("Error".to_string()))
                    }
                }
            }
            Err(e) => {
                println!("读取响应失败!");
                Err(HttpError::CustomError("http error".to_string()))
            }
        }
    }


    #[tauri::command]
    pub async fn get_chat_room_list(state: State<'_, AuthHeader>, app_handle: AppHandle<Wry>) ->
                                                                                              Result<HttpResult<Vec<ChatRoom>>,HttpError>{
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

    #[tauri::command]
    pub async fn get_room_info(roomId:String,state: State<'_, AuthHeader>, app_handle:
    AppHandle<Wry>)
        -> Result<HttpResult<ChatRoom>,HttpError>{
        match http_get::<ChatRoom>(format!("/chat-room/{}",roomId), state, app_handle).await {
            Ok(res) => {
                println!("{:?}", res);
                Ok(res)
            }
            Err(e) => {
                println!("调用失败 -> /chat-room/{}",roomId);
                Err(e)
            }
        }

    }




    #[cfg(test)]
    mod test {
        use crate::http::{decode_msg, encode_msg, get_public_key};

        #[test]
        fn tes_rsa_key() {
            use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
            use rsa::pkcs8::{DecodePublicKey, EncodePublicKey, EncodePrivateKey};
            use rand::thread_rng;
            use std::path::Path;
            use rsa::pkcs1::LineEnding;
            use base64::{Engine, engine::general_purpose};

            let mut rng = rand::thread_rng();
            let bits = 1024;

            let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
            let public_key = RsaPublicKey::from(&private_key);
            let result = public_key.to_public_key_pem(LineEnding::CRLF).unwrap();
            let res2 = private_key.to_pkcs8_pem(LineEnding::CRLF).unwrap();

            /// 将生成的公钥写入指定的文件
            // public_key.write_public_key_pem_file(Path::new("C:/Users/Administrator/Desktop/public_key.txt"), LineEnding::CRLF).unwrap();
            /// 将生成的秘钥写入指定的文件
            // private_key.write_pkcs8_pem_file(Path::new("C:/Users/Administrator/Desktop/private_key.txt"), LineEnding::CRLF).unwrap();

            println!("result1 : {:?}", result);
            println!("result2 : {:?}", res2.as_str());
            println!("private key : {:?}", private_key);
            println!("public key : {:?}", public_key);

            let raw_str = b"admin";

            let encode_str = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, raw_str).unwrap();

            let base64_encode = general_purpose::STANDARD.encode(encode_str.clone());

            println!("{:?}", base64_encode);

            let decode_str = private_key.decrypt(Pkcs1v15Encrypt, &encode_str).unwrap();


            println!("{:?}", &decode_str[..]);
            println!("{:?}", &raw_str[..]);

            let str = String::from_utf8(decode_str.clone());

            println!("{:?}", str);

            let vec = general_purpose::STANDARD.decode(base64_encode).unwrap();
            assert_eq!(vec, encode_str);

            let decode_str = private_key.decrypt(Pkcs1v15Encrypt, &vec).unwrap();
            let str = String::from_utf8(decode_str.clone());
            println!("{:?}", str);
        }

        #[test]
        fn encode_and_decode() {
            use base64::{Engine, engine::general_purpose};
            use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
            use rand::thread_rng;
            use rsa::pkcs1::LineEnding;
            use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
            use std::fs::File;
            use std::io::Read;

            let mut file: File = File::open("public_key.txt").unwrap();
            let mut public_key = String::new();
            file.read_to_string(&mut public_key).expect("can't read file");
            let local_public_key = RsaPublicKey::from_public_key_pem(&public_key).unwrap();

            let mut rng = rand::thread_rng();
            let raw_string = "admin".to_string();
            let raw_str = b"admin";
            let encode_str = local_public_key.encrypt(&mut rng, Pkcs1v15Encrypt, raw_str).unwrap();
            let base64_encode = general_purpose::STANDARD.encode(encode_str);
            println!("{:?}", base64_encode);
            let decode_str = decode_msg(&base64_encode);
            assert_eq!(raw_string, decode_str);
        }
    }


}