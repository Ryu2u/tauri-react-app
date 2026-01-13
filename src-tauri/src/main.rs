// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rbatis;
extern crate core;

use std::path::PathBuf;
use std::thread;
use std::thread::{sleep, spawn};
use std::time::Duration;
use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Icon, Manager, Menu, State, Submenu, SystemTray,
    SystemTrayEvent, SystemTrayMenu, WindowEvent, WindowMenuEvent, Wry,
};
use window_shadows::set_shadow;

use dotenv::dotenv;
use log::{error, info, LevelFilter};
use std::env;
use std::sync::Arc;

mod command;
mod http;
mod sqlite;
mod ws;

use command::{back_to_login, greet, route_to_admin};
use http::{
    check_login, get_chat_room_list, get_room_info, get_sys_time, get_user_info, login, logout,
    room_msg_list,
};
use ws::connect_websocket;

use crate::sqlite::sqlite::sqlite::delete_token_if_not_remember;
use crate::sqlite::SqliteRbatis;

pub enum ConnectedEnum {
    YES,
    NO,
}

/// 保存于 Tauri State 中
/// 用于判断当前WebSocket是否已经在连接中
/// 防止重复调用
/// 此State 也可以作为全局锁使用,不过由于是异步锁,只能在异步函数中使用，
pub struct WsConnectFlag {
    connected: Arc<tokio::sync::Mutex<ConnectedEnum>>,
}

#[tokio::main]
async fn main() {
    // 加载配置文件
    dotenv().ok();
    // 启用日志
    fast_log::init(fast_log::Config::new().console().level(LevelFilter::Info))
        .expect("rbatis  init fail");

    // 设置tauri 运行时
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // 创建系统托盘
    let tray = create_system_tray();
    // 创建系统菜单
    let menu = create_system_menu();

    // 配置Tauri
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let window = app_handle.get_window("login").unwrap();
            set_shadow(&window, true).unwrap();
            let ws_connect_flag = WsConnectFlag {
                connected: Arc::new(tokio::sync::Mutex::new(ConnectedEnum::NO)),
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
            logout,
            get_user_info,
            get_chat_room_list,
            get_room_info,
            check_login,
            room_msg_list,
            get_sys_time
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
            event
                .window()
                .app_handle()
                .emit_all("to-url-page", "https://chat.openai.com/")
                .unwrap();
        }
        "bing" => {
            event
                .window()
                .app_handle()
                .emit_all("to-url-page", "https://www.bing.com")
                .unwrap();
        }
        "open" => {
            //获取本地文件路径
            tauri::api::dialog::FileDialogBuilder::new().pick_file(|file_path| {
                info!("file path: {:?}", file_path);
            });
        }
        "add" => {
            let window = event.window();
            let app_handle = window.app_handle();

            if let Some(win) = app_handle.get_window("test") {
                win.show().unwrap();
            } else {
                thread::spawn(move || {
                    let new_window = tauri::WindowBuilder::new(
                        &app_handle,
                        "gpt",
                        tauri::WindowUrl::External("https://chat.openai.com/".parse().unwrap()),
                    )
                    .build()
                    .expect("failed to build window");
                    new_window.show().unwrap();
                    let window_copy = new_window.clone();
                    new_window.on_window_event(move |event| match event {
                        WindowEvent::CloseRequested { .. } => {
                            // 阻止窗口关闭
                            window_copy.hide().unwrap();
                        }
                        _ => {}
                    });
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
                    info!("hide clicked");
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
                    info!("quit clicked");
                    let windows = app_handle.windows();
                    for key in windows.keys() {
                        let handle = app_handle.clone();
                        let window_opt = windows.get(key);
                        if let Some(window) = window_opt {
                            tauri::api::dialog::confirm(
                                Some(&window),
                                "Tauri",
                                "确定要退出吗?",
                                move |answer| {
                                    // do something with `answer`
                                    if answer {
                                        tauri::async_runtime::block_on(async move {
                                            let option_state: Option<State<'_, SqliteRbatis>> =
                                                handle.try_state();
                                            if let Some(sql_state) = option_state {
                                                delete_token_if_not_remember(sql_state).await
                                            } else {
                                                error!("无法获取rbatis");
                                            }
                                            std::process::exit(0);
                                        });
                                    }
                                },
                            );
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

            info!("left click menu");
        }
        SystemTrayEvent::RightClick { .. } => {
            info!("right click menu");
        }
        SystemTrayEvent::DoubleClick { .. } => {
            info!("double click menu");
        }
        _ => {}
    }
}

/// 托盘图标闪烁并播放提示音
fn system_tray_flicker(app_handle: &AppHandle<Wry>) {
    use rodio::{Decoder, Source};
    use std::fs::File;
    use std::io::BufReader;

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
        let none_icon = Icon::Rgba {
            rgba: vec![0, 0, 0, 1],
            width: 1,
            height: 1,
        };
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

/// 初始化体统托盘
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

/// 创建系统菜单
fn create_system_menu() -> Menu {
    let quit = CustomMenuItem::new("quit", "退出");
    let hide = CustomMenuItem::new("hide", "最小化");
    let add_window = CustomMenuItem::new("add", "新增窗口");
    let submenu_window = Submenu::new(
        "window",
        Menu::new()
            .add_item(add_window)
            .add_item(hide)
            .add_item(quit),
    );

    let submenu_file = Submenu::new(
        "file",
        Menu::new().add_item(CustomMenuItem::new("open", "打开文件")),
    );

    let submenu_tab = Submenu::new(
        "tab",
        Menu::new()
            .add_item(CustomMenuItem::new("gpt", "ChatGPT"))
            .add_item(CustomMenuItem::new("bing", "Bing")),
    );

    let menu = Menu::new()
        .add_submenu(submenu_file)
        .add_submenu(submenu_tab)
        .add_submenu(submenu_window);

    menu
}
