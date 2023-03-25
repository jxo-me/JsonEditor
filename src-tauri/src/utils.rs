use anyhow::Result;
use log::{error, info};

use std::{
  collections::HashMap,
  fs::{self, File},
  path::{Path, PathBuf},
  process::Command,
};

use tauri::updater::UpdateResponse;
use tauri::{utils::config::Config, AppHandle, Manager, Wry};

pub fn app_root() -> PathBuf {
  tauri::api::path::home_dir().unwrap().join(".jsonEditor")
}

pub fn get_tauri_conf() -> Option<Config> {
  let config_file = include_str!("../tauri.conf.json");
  let config: Config = serde_json::from_str(config_file).expect("failed to parse tauri.conf.json");
  Some(config)
}

pub fn exists(path: &Path) -> bool {
  Path::new(path).exists()
}

pub fn convert_path(path_str: &str) -> String {
  if cfg!(target_os = "windows") {
    path_str.replace('/', "\\")
  } else {
    String::from(path_str)
  }
}

pub fn create_file(path: &Path) -> Result<File> {
  if let Some(p) = path.parent() {
    fs::create_dir_all(p)?
  }
  File::create(path).map_err(Into::into)
}

pub fn open_file(path: PathBuf) {
  let pathname = convert_path(path.to_str().unwrap());
  info!("open_file: {}", pathname);
  #[cfg(target_os = "macos")]
  Command::new("open")
    .arg("-R")
    .arg(pathname)
    .spawn()
    .unwrap();

  #[cfg(target_os = "windows")]
  Command::new("explorer.exe")
    .arg("/select,")
    .arg(pathname)
    .spawn()
    .unwrap();

  // https://askubuntu.com/a/31071
  #[cfg(target_os = "linux")]
  Command::new("xdg-open").arg(pathname).spawn().unwrap();
}

pub fn clear_conf(app: &tauri::AppHandle) {
  let root = app_root();
  let msg = format!(
    "Path: {}\n
    Are you sure you want to clear all JsonEditor configurations? Performing this operation data can not be restored, please back up in advance.\n
    Note: The application will exit automatically after the configuration cleanup!",
    root.to_string_lossy()
  );
  tauri::api::dialog::ask(
    app.get_window("core").as_ref(),
    "Clear Config",
    msg,
    move |is_ok| {
      if is_ok {
        fs::remove_dir_all(root).unwrap();
        std::process::exit(0);
      }
    },
  );
}

pub async fn get_data(
  url: &str,
  app: Option<&tauri::AppHandle>,
) -> Result<Option<String>, reqwest::Error> {
  let res = reqwest::get(url).await?;
  let is_ok = res.status() == 200;
  let body = res.text().await?;

  if is_ok {
    Ok(Some(body))
  } else {
    error!("quick_http: {}", body);
    if let Some(v) = app {
      tauri::api::dialog::message(v.get_window("core").as_ref(), "JsonEditor HTTP", body);
    }
    Ok(None)
  }
}

pub fn run_check_update(app: AppHandle<Wry>, silent: bool, has_msg: Option<bool>) {
  info!("run_check_update: silent={} has_msg={:?}", silent, has_msg);
  tauri::async_runtime::spawn(async move {
    if let Ok(update_resp) = app.updater().check().await {
      if update_resp.is_update_available() {
        if silent {
          tauri::async_runtime::spawn(async move {
            silent_install(app, update_resp).await.unwrap();
          });
        } else {
          tauri::async_runtime::spawn(async move {
            prompt_for_install(app, update_resp).await.unwrap();
          });
        }
      } else if let Some(v) = has_msg {
        if v {
          tauri::api::dialog::message(
            app.app_handle().get_window("core").as_ref(),
            "JsonEditor",
            "Your JsonEditor is up to date",
          );
        }
      }
    }
  });
}

// Copy private api in tauri/updater/mod.rs. TODO: refactor to public api
// Prompt a dialog asking if the user want to install the new version
// Maybe we should add an option to customize it in future versions.
pub async fn prompt_for_install(app: AppHandle<Wry>, update: UpdateResponse<Wry>) -> Result<()> {
  info!("prompt_for_install");
  let windows = app.windows();
  let parent_window = windows.values().next();
  let package_info = app.package_info().clone();

  let body = update.body().unwrap();
  // todo(lemarier): We should review this and make sure we have
  // something more conventional.
  let should_install = tauri::api::dialog::blocking::ask(
    parent_window,
    format!(r#"A new version of {} is available! "#, package_info.name),
    format!(
      r#"{} {} is now available -- you have {}.

Would you like to install it now?

Release Notes:
{}"#,
      package_info.name,
      update.latest_version(),
      package_info.version,
      body
    ),
  );

  if should_install {
    // Launch updater download process
    // macOS we display the `Ready to restart dialog` asking to restart
    // Windows is closing the current App and launch the downloaded MSI when ready (the process stop here)
    // Linux we replace the AppImage by launching a new install, it start a new AppImage instance, so we're closing the previous. (the process stop here)
    update.download_and_install().await?;

    // Ask user if we need to restart the application
    let should_exit = tauri::api::dialog::blocking::ask(
      parent_window,
      "Ready to Restart",
      "The installation was successful, do you want to restart the application now?",
    );
    if should_exit {
      app.restart();
    }
  }

  Ok(())
}

pub async fn silent_install(app: AppHandle<Wry>, update: UpdateResponse<Wry>) -> Result<()> {
  info!("silent_install");
  let windows = app.windows();
  let parent_window = windows.values().next();

  // Launch updater download process
  // macOS we display the `Ready to restart dialog` asking to restart
  // Windows is closing the current App and launch the downloaded MSI when ready (the process stop here)
  // Linux we replace the AppImage by launching a new install, it start a new AppImage instance, so we're closing the previous. (the process stop here)
  update.download_and_install().await?;

  // Ask user if we need to restart the application
  let should_exit = tauri::api::dialog::blocking::ask(
    parent_window,
    "Ready to Restart",
    "The silent installation was successful, do you want to restart the application now?",
  );
  if should_exit {
    app.restart();
  }

  Ok(())
}

pub fn is_hidden(entry: &walkdir::DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.starts_with('.'))
    .unwrap_or(false)
}

pub fn vec_to_hashmap(
  vec: impl Iterator<Item = serde_json::Value>,
  key: &str,
  map: &mut HashMap<String, serde_json::Value>,
) {
  for v in vec {
    if let Some(kval) = v.get(key).and_then(serde_json::Value::as_str) {
      map.insert(kval.to_string(), v);
    }
  }
}
