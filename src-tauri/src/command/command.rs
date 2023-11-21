pub mod command {
  use std::thread;
  use tauri::{Manager, Window, Wry};
  use tauri::{AppHandle};
  use window_shadows::set_shadow;

  #[tauri::command]
  pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
  }

  #[tauri::command]
  pub fn route_to_admin(app_handle: AppHandle<Wry>) {
    if let Some(login_window) = app_handle.get_window("login") {
      thread::spawn(move || {
        let main_window = get_main_window(&app_handle);
        let _ = set_shadow(&main_window, true);
        main_window.show().unwrap();
        login_window.close().unwrap();
      });
    } else {
      panic!("login window is not exists");
    }
  }

  #[tauri::command]
  pub fn back_to_login(app_handle: AppHandle<Wry>) {
    if let Some(main_window) = app_handle.get_window("main") {
      thread::spawn(move || {
        let login_window = get_login_window(&app_handle);
        login_window.show().unwrap();
        main_window.close().unwrap();
      });
    } else {
      panic!("window main is not exists");
    }
  }

  fn get_login_window(app_handle: &AppHandle<Wry>) -> Window<Wry> {
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
    new_window
  }

  fn get_main_window(app_handle: &AppHandle<Wry>) -> Window<Wry> {
    let mut builder = tauri::WindowBuilder::new(app_handle,
                                                "main",
                                                tauri::WindowUrl::App("/admin".into()),
    );
    builder = builder.
      inner_size(900f64, 700f64)
      .min_inner_size(700f64, 500f64)
      .center()
      .resizable(true)
      .decorations(false)
      .title("Home");
    let main_window = builder.build().expect("can not create window main");
    main_window
  }
}

