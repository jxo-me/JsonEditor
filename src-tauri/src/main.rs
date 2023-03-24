#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
                let window = app.get_window("main");
                match window {
                    Some(window) => match window.is_visible().expect("winVisible") {
                        true => {
                            // window.hide().expect("winHide");
                            return;
                        }
                        false => {
                            window.show().expect("winShow");
                        },
                    },
                    None => return,
                };
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "hide" => {
                        println!("system tray received a hide click");
                        let window = app.get_window("main");
                        match window {
                            Some(window) => match window.is_visible().expect("winVisible") {
                                true => {
                                    item_handle.set_title("Show").unwrap();
                                    window.hide().expect("winHide");
                                    return;
                                }
                                false => {
                                    item_handle.set_title("Hide").unwrap();
                                    window.show().expect("winShow");
                                },
                            },
                            None => return,
                        };
                    }
                    "quit" => {
                        println!("system tray received a quit click");
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
