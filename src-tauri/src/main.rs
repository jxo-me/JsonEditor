#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app;
mod conf;
mod utils;

use app::{cmd, menu, setup};
use conf::AppConf;
use log::info;

use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::{
  LogTarget,
};

fn main() {
  let app_conf = AppConf::read().write();

  let context = tauri::generate_context!();

  let mut log = tauri_plugin_log::Builder::default()
  .targets([
    // LogTarget::LogDir,
    // LOG PATH: ~/.quickType/quickType.log
    LogTarget::Folder(utils::app_root()),
    LogTarget::Stdout,
    LogTarget::Webview,
  ])
  .level(log::LevelFilter::Debug);

  let mut builder = tauri::Builder::default()
    .plugin(log.build())
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      None,
    ))
    .invoke_handler(tauri::generate_handler![cmd::run_check_update,])
    .setup(setup::init)
    .menu(menu::init());

  if app_conf.tray {
    builder = builder.system_tray(menu::tray_menu());
  }
  builder
    .on_menu_event(menu::menu_handler)
    .on_system_tray_event(menu::tray_handler)
    .on_window_event(move |event| {
      info!("on_window_event...");
      if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
        let win = event.window().clone();
        let app_conf = AppConf::read();
        if win.label() == "core" {
          if app_conf.isinit {
            tauri::api::dialog::ask(
              Some(event.window()),
              "",
              "Do you want to exit the application when you click the [x] button?",
              move |is_ok| {
                app_conf
                  .amend(serde_json::json!({ "isinit" : false, "main_close": is_ok }))
                  .write();
                if is_ok {
                  std::process::exit(0);
                } else {
                  win.minimize().unwrap();
                }
              },
            );
          } else if app_conf.main_close {
            std::process::exit(0);
          } else {
            win.minimize().unwrap();
          }
        } else {
          event.window().close().unwrap();
        }
        api.prevent_close();
      }
    })
    .run(context)
    .expect("error while running JsonEditor application")
  // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
  // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  // let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  // let tray_menu = SystemTrayMenu::new()
  //   .add_item(hide)
  //   .add_native_item(SystemTrayMenuItem::Separator)
  //   .add_item(quit);

  // let system_tray = SystemTray::new().with_menu(tray_menu);

  // tauri::Builder::default()
  //   .system_tray(system_tray)
  //   .on_system_tray_event(|app, event| match event {
  //     SystemTrayEvent::LeftClick {
  //       position: _,
  //       size: _,
  //       ..
  //     } => {
  //       println!("system tray received a left click");
  //     }
  //     SystemTrayEvent::RightClick {
  //       position: _,
  //       size: _,
  //       ..
  //     } => {
  //       println!("system tray received a right click");
  //     }
  //     SystemTrayEvent::DoubleClick {
  //       position: _,
  //       size: _,
  //       ..
  //     } => {
  //       println!("system tray received a double click");
  //       let window = app.get_window("main");
  //       match window {
  //         Some(window) => match window.is_visible().expect("winVisible") {
  //           true => {
  //             // window.hide().expect("winHide");
  //             return;
  //           }
  //           false => {
  //             window.show().expect("winShow");
  //           }
  //         },
  //         None => return,
  //       };
  //     }
  //     SystemTrayEvent::MenuItemClick { id, .. } => {
  //       let item_handle = app.tray_handle().get_item(&id);
  //       match id.as_str() {
  //         "hide" => {
  //           println!("system tray received a hide click");
  //           let window = app.get_window("main");
  //           match window {
  //             Some(window) => match window.is_visible().expect("winVisible") {
  //               true => {
  //                 item_handle.set_title("Show").unwrap();
  //                 window.hide().expect("winHide");
  //                 return;
  //               }
  //               false => {
  //                 item_handle.set_title("Hide").unwrap();
  //                 window.show().expect("winShow");
  //               }
  //             },
  //             None => return,
  //           };
  //         }
  //         "quit" => {
  //           println!("system tray received a quit click");
  //           std::process::exit(0);
  //         }
  //         _ => {}
  //       }
  //     }
  //     _ => {}
  //   })
  //   .invoke_handler(tauri::generate_handler![greet])
  //   .run(tauri::generate_context!())
  //   .expect("error while running tauri application");
}
