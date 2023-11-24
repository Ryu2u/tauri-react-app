pub mod structs {
    use std::fmt::{Display, Formatter, write};
    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};

    // create the error type that represents all errors possible in our program
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
        pub key: String,
    }

    impl Clone for AuthHeader {
        fn clone(&self) -> Self {
            AuthHeader {
                Authorization: self.Authorization.clone(),
                refresh_token: self.refresh_token.clone(),
                key: self.key.clone(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_snake_case)]
    pub struct User {
        id: i32,
        username: String,
        nickName: String,
        password: String,
        avatarPath: String,
        locked: bool,
        createdBy: i32,
        createdTime: i64,
    }

}