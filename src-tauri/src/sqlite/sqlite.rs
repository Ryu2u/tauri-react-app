pub mod sqlite {
    use crate::sqlite::{AuthHeader, SqliteRbatis};
    use log::{error, info};
    use rbatis::RBatis;
    use tauri::State;

    pub async fn get_token(rb: &RBatis) -> String {
        let res = AuthHeader::get_token(rb).await;
        if let Ok(data) = res {
            if data.len() > 0 {
                let x = data.get(0).unwrap();
                x.Authorization.clone()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    }

    pub async fn delete_token(sql_state: State<'_, SqliteRbatis>) {
        let sql_state = sql_state.db.lock().await;
        let res = AuthHeader::delete_token(&*sql_state).await;
        res.unwrap();
    }

    pub async fn delete_token_if_not_remember(sql_state: State<'_, SqliteRbatis>) {
        info!("尝试删除token");
        let res = AuthHeader::get_token(&*sql_state.db.clone().lock().await).await;
        if let Ok(data) = res {
            if data.len() > 0 {
                let x = data.get(0).unwrap();
                if x.remember_me == 0 {
                    info!("删除token。。。");
                    delete_token(sql_state).await;
                }
            }
        } else {
            error!("获取token 失败!");
        }
    }

    #[cfg(test)]
    mod test {
        use crate::sqlite::{AuthHeader, User};
        use dotenv::dotenv;
        use log::info;
        use rbatis::RBatis;
        use rbdc_sqlite::SqliteDriver;
        use std::env;

        #[tokio::test]
        async fn test_rbatis() {
            dotenv().ok();
            /// enable log crate to show sql logs
            fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
            /// initialize rbatis. also you can call rb.clone(). this is  an Arc point
            let rb = RBatis::new();
            /// connect to database
            // sqlite
            let sqlite_url = env::var("SQLITE_URL").unwrap();
            rb.init(SqliteDriver {}, sqlite_url.as_str()).unwrap();

            let data = User::select_by_id(&rb, 1).await;
            info!("{:?}", data);
        }

        #[tokio::test]
        async fn test_get_token() {
            dotenv().ok();
            fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
            let rb = RBatis::new();
            let sqlite_url = env::var("SQLITE_URL").unwrap();
            rb.init(SqliteDriver {}, sqlite_url.as_str()).unwrap();
            let data = AuthHeader::get_token(&rb).await;
            info!("{:?}", data);
        }

        #[tokio::test]
        async fn test_delete_token() {
            dotenv().ok();
            fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
            let rb = RBatis::new();
            let sqlite_url = env::var("SQLITE_URL").unwrap();
            rb.init(SqliteDriver {}, sqlite_url.as_str()).unwrap();
            let data = AuthHeader::delete_token(&rb).await;
            info!("{:?}", data);
        }
    }
}
