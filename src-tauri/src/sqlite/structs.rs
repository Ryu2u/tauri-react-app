pub mod structs {
    use std::fmt::{Debug, Display, Formatter, write};
    use rbatis::crud;
    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};

    /// 自定义Http异常
    #[derive(Debug)]
    pub enum HttpError {
        CustomError(String),
        RequestError(StatusCode),
    }

    impl Display for HttpError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                HttpError::RequestError(e) => {
                    eprintln!("{:?}", e);
                }
                HttpError::CustomError(e) => {
                    eprintln!("{}", e);
                }
            }
            println!("发生错误!");
            write!(f, "custom error")
        }
    }

    // we must manually implement serde::Serialize
    impl serde::Serialize for HttpError {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
        {
            serializer.serialize_str("custom error")
        }
    }


    #[derive(Serialize, Deserialize, Debug)]
    pub struct HttpResult<T> {
        pub code: i32,
        pub msg: String,
        pub data: Option<T>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct AuthHeader {
        pub Authorization: String,
        pub refresh_token: String,
        pub remember_me: i32,
        pub key: String,
    }
    crud!(AuthHeader{});
    impl_select!(AuthHeader{get_token() => "`limit 1`"});
    impl_delete!(AuthHeader{delete_token() => "`where 1=1`"});

    impl Clone for AuthHeader {
        fn clone(&self) -> Self {
            AuthHeader {
                Authorization: self.Authorization.clone(),
                refresh_token: self.refresh_token.clone(),
                remember_me: self.remember_me,
                key: self.key.clone(),
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct User {
        id: i32,
        username: String,
        nickName: String,
        avatarPath: String,
        createdBy: i32,
        createdTime: i64,
    }
    crud!(User{},"tb_user");
    impl_select!(User{select_by_id(id:i32) => "`where id = #{id}`"},"tb_user");



    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct ChatRoom {
        id: i32,
        isGroup: bool,
        roomName: String,
        roomAvatar: String,
        isTop: bool,
        isView: bool,
    }
    crud!(ChatRoom{},"tb_chat_room");


    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct ChatMessage{
        id:String,
        roomId:i32,
        senderId:i32,
        senderName:String,
        content:String,
        sendTime:i64,
        createdBy:i32,
        createdTime:i64
    }




}