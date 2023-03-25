use crate::utils;
use log::error;
use std::{fs, path::PathBuf};
use tauri::{api, command, AppHandle, Manager};

#[command]
pub fn download(app: AppHandle, name: String, blob: Vec<u8>) {
  let win = app.app_handle().get_window("core");
  let path = utils::app_root().join(PathBuf::from(name));
  utils::create_file(&path).unwrap();
  fs::write(&path, blob).unwrap();
  tauri::api::dialog::message(
    win.as_ref(),
    "Save File",
    format!("PATH: {}", path.display()),
  );
}

#[command]
pub fn save_file(app: AppHandle, name: String, content: String) {
  let win = app.app_handle().get_window("core");
  let path = utils::app_root().join(PathBuf::from(name));
  utils::create_file(&path).unwrap();
  fs::write(&path, content).unwrap();
  tauri::api::dialog::message(
    win.as_ref(),
    "Save File",
    format!("PATH: {}", path.display()),
  );
}

#[command]
pub fn run_check_update(app: AppHandle, silent: bool, has_msg: Option<bool>) {
  utils::run_check_update(app, silent, has_msg);
}
