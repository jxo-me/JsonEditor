use crate::{
  conf::{self, AppConf},
  utils,
};

use tauri::{
  AppHandle, CustomMenuItem, Manager, Menu, MenuItem, Submenu, SystemTray, SystemTrayEvent,
  SystemTrayMenu, SystemTrayMenuItem, WindowMenuEvent,
};

// --- Menu
pub fn init() -> Menu {
  let app_conf = AppConf::read();
  let name = "JsonEditor";
  let app_menu = Submenu::new(
    name,
    Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuItem::About(name.into(), AboutMetadata::default()).into(),
      #[cfg(not(target_os = "macos"))]
      CustomMenuItem::new("about".to_string(), "About QuickType").into(),
      CustomMenuItem::new("check_update".to_string(), "Check for Updates").into(),
      MenuItem::Services.into(),
      MenuItem::Hide.into(),
      MenuItem::HideOthers.into(),
      MenuItem::ShowAll.into(),
      MenuItem::Separator.into(),
      MenuItem::Quit.into(),
    ]),
  );

  let edit_menu = Submenu::new(
    "Edit",
    Menu::new()
      .add_native_item(MenuItem::Undo)
      .add_native_item(MenuItem::Redo)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Cut)
      .add_native_item(MenuItem::Copy)
      .add_native_item(MenuItem::Paste)
      .add_native_item(MenuItem::SelectAll),
  );

  let help_menu = Submenu::new(
    "Help",
    Menu::new()
      .add_item(CustomMenuItem::new(
        "jsoneditor_log".to_string(),
        "JsonEditor Log",
      ))
      .add_item(CustomMenuItem::new("update_log".to_string(), "Update Log")), // .add_item(
                                                                              //   CustomMenuItem::new("dev_tools".to_string(), "Toggle Developer Tools")
                                                                              //     .accelerator("CmdOrCtrl+Shift+I"),
                                                                              // ),
  );

  Menu::new()
    .add_submenu(app_menu)
    .add_submenu(edit_menu)
    .add_submenu(help_menu)
}

// --- Menu Event
pub fn menu_handler(event: WindowMenuEvent<tauri::Wry>) {
  let win = Some(event.window()).unwrap();
  let app = win.app_handle();
  let menu_id = event.menu_item_id();
  let menu_handle = win.menu_handle();
  match menu_id {
    // App
    "about" => {
      let tauri_conf = utils::get_tauri_conf().unwrap();
      tauri::api::dialog::message(
        app.get_window("core").as_ref(),
        "JsonEditor",
        format!("Version {}", tauri_conf.package.version.unwrap()),
      );
    }
    "check_update" => {
      println!("menu_handler check_update");
      utils::run_check_update(app, false, None);
    }
    // Help
    "quicktype_log" => utils::open_file(utils::app_root().join("quicktype.log")),
    "update_log" => open(&app, conf::UPDATE_LOG_URL.to_string()),
    _ => (),
  }
}

// --- SystemTray Menu
pub fn tray_menu() -> SystemTray {
  if cfg!(target_os = "macos") {
    let mut tray_menu = SystemTrayMenu::new();

    if AppConf::read().hide_dock_icon {
      tray_menu = tray_menu.add_item(CustomMenuItem::new(
        "show_dock_icon".to_string(),
        "Show Dock Icon",
      ));
    } else {
      tray_menu = tray_menu
        .add_item(CustomMenuItem::new(
          "hide_dock_icon".to_string(),
          "Hide Dock Icon",
        ))
        .add_item(CustomMenuItem::new("show_core".to_string(), "Show Window"));
    }

    SystemTray::new().with_menu(
      tray_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
    )
  } else {
    SystemTray::new().with_menu(
      SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show_core".to_string(), "Show Window"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
    )
  }
}

// --- SystemTray Event
pub fn tray_handler(handle: &AppHandle, event: SystemTrayEvent) {
  let app = handle.clone();

  match event {
    SystemTrayEvent::LeftClick { .. } => {
      let app_conf = AppConf::read();

      if !app_conf.hide_dock_icon {
        if let Some(core_win) = handle.get_window("core") {
          core_win.minimize().unwrap();
        }
      }

      if let Some(tray_win) = handle.get_window("tray") {
        if tray_win.is_visible().unwrap() {
          tray_win.hide().unwrap();
        } else {
          tray_win.show().unwrap();
        }
      }
    }
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
      "restart" => tauri::api::process::restart(&handle.env()),
      "show_dock_icon" => {
        AppConf::read()
          .amend(serde_json::json!({ "hide_dock_icon": false }))
          .write()
          .restart(app);
      }
      "hide_dock_icon" => {
        let app_conf = AppConf::read();
        if !app_conf.hide_dock_icon {
          app_conf
            .amend(serde_json::json!({ "hide_dock_icon": true }))
            .write()
            .restart(app);
        }
      }
      "show_core" => {
        if let Some(core_win) = app.get_window("core") {
          let tray_win = app.get_window("tray").unwrap();
          if !core_win.is_visible().unwrap() {
            core_win.show().unwrap();
            core_win.set_focus().unwrap();
            tray_win.hide().unwrap();
          }
        };
      }
      "quit" => std::process::exit(0),
      _ => (),
    },
    _ => (),
  }
}

pub fn open(app: &AppHandle, path: String) {
  tauri::api::shell::open(&app.shell_scope(), path, None).unwrap();
}