use log::{error, info};
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf};
use tauri::{Manager, Theme};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

use crate::utils::{app_root, create_file, exists};

pub const APP_CONF_PATH: &str = "editor.conf.json";
pub const UPDATE_LOG_URL: &str = "https://github.com/jxo-me/JsonEditor/blob/main/UPDATE_LOG.md";


macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
      #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
      pub struct $name {
        $(pub $field: $t),*
      }
    }
}

pub_struct!(AppConf {
    titlebar: bool,
    hide_dock_icon: bool,
    // macOS and Windows: light / dark / system
    theme: String,
    // auto update policy: prompt / silent / disable
    auto_update: String,
    stay_on_top: bool,
    save_window_state: bool,
    global_shortcut: Option<String>,
    speech_lang: String,

    // Main Window
    isinit: bool,
    popup_search: bool,
    main_close: bool,
    main_dashboard: bool,
    main_width: f64,
    main_height: f64,

    // Tray Window
    tray_width: f64,
    tray_height: f64,
    tray: bool,
    tray_dashboard: bool,
});

impl AppConf {
  pub fn new() -> Self {
    info!("conf_init");
    Self {
      titlebar: true,
      hide_dock_icon: false,
      save_window_state: false,
      theme: "light".into(),
      auto_update: "prompt".into(),
      #[cfg(target_os = "macos")]
      speech_lang: "com.apple.eloquence.en-US.Rocko".into(),
      #[cfg(not(target_os = "macos"))]
      speech_lang: "".into(),
      tray: true,
      popup_search: false,
      isinit: true,
      main_close: false,
      stay_on_top: false,
      main_dashboard: false,
      tray_dashboard: false,
      main_width: 1440.0,
      main_height: 900.0,
      tray_width: 1440.0,
      tray_height: 900.0,
      global_shortcut: None,
    }
  }
  pub fn file_path() -> PathBuf {
    app_root().join(APP_CONF_PATH)
  }

  pub fn read() -> Self {
    match std::fs::read_to_string(Self::file_path()) {
      Ok(v) => {
        if let Ok(v2) = serde_json::from_str::<AppConf>(&v) {
          v2
        } else {
          error!("conf_read_parse_error");
          Self::default()
        }
      }
      Err(err) => {
        error!("conf_read_error: {}", err);
        Self::default()
      }
    }
  }

  pub fn write(self) -> Self {
    let path = &Self::file_path();
    if !exists(path) {
      create_file(path).unwrap();
      info!("conf_create");
    }
    if let Ok(v) = serde_json::to_string_pretty(&self) {
      std::fs::write(path, v).unwrap_or_else(|err| {
        error!("conf_write: {}", err);
        Self::default().write();
      });
    } else {
      error!("conf_ser");
    }
    self
  }

  pub fn amend(self, json: Value) -> Self {
    let val = serde_json::to_value(&self).unwrap();
    let mut config: BTreeMap<String, Value> = serde_json::from_value(val).unwrap();
    let new_json: BTreeMap<String, Value> = serde_json::from_value(json).unwrap();

    for (k, v) in new_json {
      config.insert(k, v);
    }

    match serde_json::to_string_pretty(&config) {
      Ok(v) => match serde_json::from_str::<AppConf>(&v) {
        Ok(v) => v,
        Err(err) => {
          error!("conf_amend_parse: {}", err);
          self
        }
      },
      Err(err) => {
        error!("conf_amend_str: {}", err);
        self
      }
    }
  }

  #[cfg(target_os = "macos")]
  pub fn titlebar(self) -> TitleBarStyle {
    if self.titlebar {
      TitleBarStyle::Transparent
    } else {
      TitleBarStyle::Overlay
    }
  }

  pub fn theme_mode() -> Theme {
    match Self::get_theme().as_str() {
      "system" => match dark_light::detect() {
        // Dark mode
        dark_light::Mode::Dark => Theme::Dark,
        // Light mode
        dark_light::Mode::Light => Theme::Light,
        // Unspecified
        dark_light::Mode::Default => Theme::Light,
      },
      "dark" => Theme::Dark,
      _ => Theme::Light,
    }
  }

  pub fn get_theme() -> String {
    Self::read().theme.to_lowercase()
  }

  pub fn get_auto_update(self) -> String {
    self.auto_update.to_lowercase()
  }

  pub fn theme_check(self, mode: &str) -> bool {
    self.theme.to_lowercase() == mode
  }

  pub fn restart(self, app: tauri::AppHandle) {
    tauri::api::process::restart(&app.env());
  }
}

impl Default for AppConf {
  fn default() -> Self {
    Self::new()
  }
}
