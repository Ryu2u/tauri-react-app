pub mod command {
    use tauri::{Manager, State, Window, Wry};
    use tauri::{AppHandle};
    use window_shadows::set_shadow;
    use crate::WsConnectFlag;

    #[tauri::command]
    pub fn greet(name: &str) -> String {
        format!("Hello {}", name)
    }

    /// 登录完成操作，跳转到主页面
    #[tauri::command]
    pub async fn route_to_admin(app_handle: AppHandle<Wry>) {
        let lock_state:State<'_,WsConnectFlag> = app_handle.state();
        let lock = lock_state.connected.lock().await;
        if let Some(window) = app_handle.get_window("main") {
            if !window.is_visible().unwrap() {
                window.show().unwrap();
            }
            return;
        }
        if let Some(login_window) = app_handle.get_window("login") {
            let main_window = get_main_window(&app_handle);
            main_window.show().unwrap();
            login_window.close().unwrap();
        } else {
            panic!("login window is not exists");
        }
        match *lock {
            _ => {
                println!("释放创建main窗口锁!");
            }
        }
        drop(lock);
    }

    /// 创建Login 窗口并关闭其他所有窗口
    #[tauri::command]
    pub fn back_to_login(app_handle: AppHandle<Wry>) {
        let login_window = get_login_window(&app_handle);
        login_window.show().unwrap();
        let windows = app_handle.windows();
        for key in windows.keys() {
            let window_opt = windows.get(key);
            if let Some(window) = window_opt {
                if window.label() != "login" {
                    window.close().unwrap();
                }
            }
        }
    }


    /// 创建login 窗口
    fn get_login_window(app_handle: &AppHandle<Wry>) -> Window<Wry> {
        if let Some(main_window) = app_handle.get_window("login") {
            main_window
        } else {
            let mut builder = tauri::WindowBuilder::new(app_handle,
                                                        "login",
                                                        tauri::WindowUrl::App("/login".into()),
            );
            builder = builder.
                inner_size(430f64, 330f64)
                .center()
                .resizable(false)
                .decorations(false)
                .title("登录");
            let new_window = builder.build().expect("can not create window main");
            let _ = set_shadow(&new_window, true);
            new_window
        }
    }

    /// 创建main 窗口
    fn get_main_window(app_handle: &AppHandle<Wry>) -> Window<Wry> {
        println!("正在创建main窗口!!!!!!!!!!!!!!!!!");
        if let Some(main_window) = app_handle.get_window("main") {
            main_window
        } else {
            println!("创建main窗口!!!!!!!!!!!!!!!!!");
            let main_window = tauri::WindowBuilder::new(app_handle,
                                                        "main",
                                                        tauri::WindowUrl::App("/admin".into()),
            ).inner_size(900f64, 700f64)
                .min_inner_size(700f64, 500f64)
                .center()
                .resizable(true)
                .decorations(false)
                .title("Home").build().expect("can not create window main");
            let _ = set_shadow(&main_window, true);
            println!("main window created");
            main_window
        }
    }
}

