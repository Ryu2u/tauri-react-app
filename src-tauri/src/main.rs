// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rbatis;
extern crate core;

use std::path::PathBuf;
use std::thread;
use std::thread::{sleep, spawn};
use std::time::Duration;
use tauri::{AppHandle, Menu, CustomMenuItem, GlobalWindowEvent, Icon, Manager, SystemTray, Submenu, SystemTrayEvent, SystemTrayMenu, WindowEvent, WindowMenuEvent, Wry, State};
use window_shadows::set_shadow;

use std::env;
use std::sync::{Arc};
use std::sync::{Mutex};
use dotenv::dotenv;
use rbatis::RBatis;
use rbdc_sqlite::SqliteDriver;

mod command;
mod ws;
mod http;
mod sqlite;

use ws::connect_websocket;
use command::{greet, route_to_admin, back_to_login};
use http::{login, get_user_info, get_chat_room_list, get_room_info, check_login,room_msg_list};
use crate::sqlite::sqlite::sqlite::delete_token_if_not_remember;


pub enum ConnectedEnum {
    YES,
    NO,
}

/// 保存于 Tauri State 中
/// 用于判断当前WebSocket是否已经在连接中
/// 防止重复调用
pub struct WsConnectFlag {
    connected: Arc<Mutex<ConnectedEnum>>,
}


#[tokio::main]
async fn main() {
    // 加载配置文件
    dotenv().ok();
    // 创建系统托盘
    let tray = create_system_tray();
    // 创建系统菜单
    let menu = create_system_menu();
    /// enable log crate to show sql logs
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    /// initialize rbatis. also you can call rb.clone(). this is  an Arc point

    // /// connect to database
    // let sqlite_url = env::var("SQLITE_URL").unwrap();
    // rb.init(SqliteDriver {}, sqlite_url.as_str()).unwrap();

    // 设置tauri 运行时
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // 配置Tauri
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let window = app_handle.get_window("login").unwrap();
            set_shadow(&window, true).unwrap();
            let ws_connect_flag = WsConnectFlag {
                connected: Arc::new(Mutex::new(ConnectedEnum::NO))
            };
            app_handle.manage(ws_connect_flag);
            Ok(())
        })
        .menu(menu)
        .on_menu_event(|event| menu_event_handle(event))
        // 配置rust指令，可以让前端调用
        .invoke_handler(tauri::generate_handler![
            greet,
      route_to_admin,
      back_to_login,
      connect_websocket,
            login,
            get_user_info,
            get_chat_room_list,
            get_room_info,
            check_login,
            room_msg_list
    ])
        // 配置系统托盘
        .system_tray(tray)
        // 设置窗口事件
        .on_window_event(|event| window_event_handle(event))
        // 系统托盘事件
        .on_system_tray_event(|app, event| tray_menu_handle(app, event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// 工具栏菜单点击事件
fn menu_event_handle(event: WindowMenuEvent<Wry>) {
    match event.menu_item_id() {
        "gpt" => {
            event.window().app_handle().emit_all("to-url-page", "https://chat.openai.com/").unwrap();
        }
        "bing" => {
            event.window().app_handle().emit_all("to-url-page", "https://www.bing.com").unwrap();
        }
        "open" => {
            //获取本地文件路径
            tauri::api::dialog::FileDialogBuilder::new().pick_file(|file_path| {
                println!("file path: {:?}", file_path);
            });
        }
        "add" => {
            let window = event.window();
            let app_handle = window.app_handle();

            if let Some(win) = app_handle.get_window("test") {
                win.show().unwrap();
            } else {
                thread::spawn(move || {
                    let new_window = tauri::WindowBuilder::new(&app_handle,
                                                               "gpt",
                                                               tauri::WindowUrl::External
                                                                   ("https://chat.openai.com/".parse()
                                                                       .unwrap())).build().expect
                    ("failed to build window");
                    new_window.show().unwrap();
                    let window_copy = new_window.clone();
                    new_window.on_window_event(move |event| match event {
                        WindowEvent::CloseRequested { .. } => {
                            // 阻止窗口关闭
                            window_copy.hide().unwrap();
                        }
                        _ => {}
                    }
                    );
                });
            }
        }
        "hide" => {
            event.window().hide().unwrap();
        }
        "quit" => {
            event.window().close().unwrap();
        }
        _ => {}
    };
}

/// 窗口事件
fn window_event_handle(event: GlobalWindowEvent<Wry>) {
    match event.event() {
        WindowEvent::Resized(_) => {}
        WindowEvent::Moved(_) => {}
        WindowEvent::CloseRequested { api, .. } => {
            // 阻止窗口关闭
            api.prevent_close();
            event.window().hide().unwrap();
        }
        WindowEvent::Destroyed => {}
        WindowEvent::Focused(_) => {}
        WindowEvent::ScaleFactorChanged { .. } => {}
        WindowEvent::FileDrop(_) => {}
        WindowEvent::ThemeChanged(_) => {}
        _ => {}
    }
}


///系统托盘菜单点击事件
fn tray_menu_handle(app_handle: &AppHandle<Wry>, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "hide" => {
                    println!("hide clicked");
                    let windows = app_handle.windows();
                    for key in windows.keys() {
                        let window_opt = windows.get(key);
                        if let Some(window) = window_opt {
                            if window.is_visible().unwrap() {
                                window.hide().unwrap();
                            }
                        }
                    }
                }
                "quit" => {
                    println!("quit clicked");
                    let windows = app_handle.windows();
                    for key in windows.keys() {
                        let handle = app_handle.clone();
                        let window_opt = windows.get(key);
                        if let Some(window) = window_opt {
                            tauri::api::dialog::confirm(Some(&window), "Tauri", "确定要退出吗?", move |answer| {
                                // do something with `answer`
                                if answer {
                                    tauri::async_runtime::block_on( async
                                        move {
                                        let option_state: Option<State<'_, RBatis>> = handle.try_state();
                                        if let Some(sql_state) = option_state {
                                            delete_token_if_not_remember(sql_state).await
                                        }else{
                                            println!("无法获取rbatis");
                                        }
                                        std::process::exit(0);
                                    });
                                }
                            });
                        }
                    }
                }
                _ => {
                    panic!("unimplemented menu");
                }
            }
        }
        SystemTrayEvent::LeftClick { .. } => {
            let windows = app_handle.windows();
            for key in windows.keys() {
                let window_opt = windows.get(key);
                if let Some(window) = window_opt {
                    if !window.is_visible().unwrap() {
                        window.show().unwrap();
                    }
                    if !window.is_focused().unwrap() {
                        window.set_focus().unwrap();
                    }
                }
            }

            println!("left click menu");
        }
        SystemTrayEvent::RightClick { .. } => {
            println!("right click menu");
        }
        SystemTrayEvent::DoubleClick { .. } => {
            println!("double click menu");
        }
        _ => {}
    }
}


fn system_tray_flicker(app_handle: &AppHandle<Wry>) {
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, Source};

    let window = app_handle.get_window("main").unwrap();

    if !window.is_visible().unwrap() {
        spawn(|| {
            let audio = File::open("audio/reminder.mp3").unwrap();
            let file = BufReader::new(audio);
            let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            let source = Decoder::new(file).unwrap();
            stream_handle.play_raw(source.convert_samples()).unwrap();
            sleep(Duration::from_millis(600));
        });
    }
    let app = app_handle.clone();
    spawn(move || {
        let handle = app.tray_handle();
        let none_icon = Icon::Rgba { rgba: vec![0, 0, 0, 1], width: 1, height: 1 };
        let origin_icon = Icon::File(PathBuf::from("icons/icon.ico"));
        loop {
            if window.is_visible().unwrap() {
                break;
            } else {
                handle.set_icon(none_icon.clone()).unwrap();
                sleep(Duration::from_millis(400));
                handle.set_icon(origin_icon.clone()).unwrap();
                sleep(Duration::from_millis(400));
            }
        }
    });
}

fn create_system_tray() -> SystemTray {
    // 创建托盘菜单
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("hide", "最小化"))
        .add_item(CustomMenuItem::new("quit", "退出"));
    // 创建系统托盘
    let tray = SystemTray::default()
        // 添加菜单
        .with_menu(tray_menu)
        // 添加托盘鼠标放上去显示的文字
        .with_tooltip("Chat");
    tray
}

fn create_system_menu() -> Menu {
    let quit = CustomMenuItem::new("quit", "退出");
    let hide = CustomMenuItem::new("hide", "最小化");
    let add_window = CustomMenuItem::new("add", "新增窗口");
    let submenu_window = Submenu::new("window", Menu::new()
        .add_item(add_window)
        .add_item(hide)
        .add_item(quit));

    let submenu_file = Submenu::new("file", Menu::new()
        .add_item(CustomMenuItem::new("open", "打开文件")),
    );

    let submenu_tab = Submenu::new("tab", Menu::new()
        .add_item(CustomMenuItem::new("gpt", "ChatGPT"))
        .add_item(CustomMenuItem::new("bing", "Bing")),
    );

    let menu = Menu::new()
        .add_submenu(submenu_file)
        .add_submenu(submenu_tab)
        .add_submenu(submenu_window);

    menu
}
