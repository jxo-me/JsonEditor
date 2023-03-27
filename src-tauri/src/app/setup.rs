use crate::{conf::AppConf, utils};
use log::{info};
use tauri::{utils::config::WindowUrl, window::WindowBuilder, App};

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
  info!("stepup");
  let app_conf = AppConf::read();
  let theme = AppConf::theme_mode();
  // let handle = app.app_handle();

  let app_conf2 = app_conf.clone();
  if app_conf.hide_dock_icon {
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
  } else {
    let app = app.handle();
    tauri::async_runtime::spawn(async move {
      let link = "index.html";
      info!("main_window: {}", link);
      let mut main_win = WindowBuilder::new(&app, "core", WindowUrl::App(link.into()))
        .title("JsonEditor")
        .resizable(true)
        .fullscreen(false)
        .inner_size(app_conf2.main_width, app_conf2.main_height)
        .theme(Some(theme))
        .always_on_top(app_conf2.stay_on_top);

      #[cfg(target_os = "macos")]
      {
        main_win = main_win
          .title_bar_style(app_conf2.clone().titlebar())
          .hidden_title(true);
      }

      main_win.build().unwrap();
    });
  }

  // auto_update
  let auto_update = app_conf.get_auto_update();
  if auto_update != "disable" {
    info!("run_check_update");
    let app = app.handle();
    utils::run_check_update(app, auto_update == "silent", None);
  }

  Ok(())
}
