#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod closer;
mod config;
mod launcher;
mod page;

use std::sync::Arc;

use closer::Closer;
use launcher::{Launcher, SearchOption};
use page::{MainData, Page};
use tauri::{
  ActivationPolicy, AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, SystemTray,
  SystemTrayEvent, SystemTrayMenu, Window, WindowEvent,
};
use tracing::{error, info};

struct State {
  launcher: Launcher,
}

#[tauri::command]
fn close(window: tauri::Window) -> Result<(), String> {
  Closer::close(&window);
  Ok(())
}

#[tauri::command]
async fn search(
  state: tauri::State<'_, State>,
  search: String,
) -> Result<Vec<SearchOption>, String> {
  Ok(state.launcher.get_options(&search).await)
}

#[tauri::command]
fn submit(
  state: tauri::State<State>,
  selection: usize,
  window: tauri::Window,
) -> Result<(), String> {
  match state.launcher.launch(selection) {
    Ok(()) => {
      Closer::close(&window);
      Ok(())
    }
    Err(err) => {
      info!("Failed to launch option {}", err);
      Err("Failed to launch".into())
    }
  }
}

fn open_settings(app: &AppHandle) -> Result<(), anyhow::Error> {
  let page = Page::Settings;
  Window::builder(app, page.id(), tauri::WindowUrl::App("index.html".into()))
    .center()
    .initialization_script(&page.init_script()?)
    .build()?;
  Ok(())
}

fn main() {
  if let Err(err) = config::init_logs() {
    error!("Failed to start logger: {}", err);
    return;
  }

  let config = match config::get_or_init_config() {
    Ok(c) => Arc::new(c),
    Err(err) => {
      info!("Failed to initialize config: {}", err);
      return;
    }
  };

  let tray_menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("settings".to_string(), "Settings"))
    .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

  tauri::Builder::default()
    .system_tray(SystemTray::new().with_menu(tray_menu))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "quit" => {
          std::process::exit(0);
        }
        "settings" => {
          if let Err(err) = open_settings(app) {
            error!("Failed to open settings: {}", err);
          }
        }
        _ => {}
      },
      _ => {}
    })
    .on_window_event(|event| match event.event() {
      WindowEvent::Focused(focused) if !focused => {
        if !focused && event.window().label() == Page::Main(MainData::default()).id() {
          Closer::close(&event.window());
        }
      }
      _ => {}
    })
    .setup(|app| {
      #[cfg(target_os = "macos")]
      app.set_activation_policy(ActivationPolicy::Accessory);

      // TODO code assumes input is 38px large, and each result is 18px with max of 10 results shown.
      let page = Page::Main(
        MainData::builder()
          .style(("OPTION_HEIGHT".into(), 18.into()))
          .style(("INPUT_HEIGHT".into(), 38.into()))
          .style(("FONT_SIZE".into(), 16.into()))
          .build()?,
      );

      Window::builder(app, page.id(), tauri::WindowUrl::App("index.html".into()))
        .inner_size(600f64, 218f64)
        .resizable(false)
        .always_on_top(true)
        .decorations(false)
        .visible(false)
        .fullscreen(false)
        .skip_taskbar(true)
        .center()
        .initialization_script(&page.init_script()?)
        .build()?;

      let handle = app.handle();
      app
        .global_shortcut_manager()
        .register("CmdOrCtrl+Space", move || {
          let win = handle
            .get_window(page.id())
            .expect("Framework should have built");
          let is_updated = match win.is_visible() {
            Ok(true) => Ok(Closer::close(&win)),
            Ok(false) => win.set_focus(),
            Err(err) => Err(err),
          };
          if let Err(err) = is_updated {
            info!("Failed to toggle window: {}", err);
          }
        })?;
      Ok(())
    })
    .manage(State {
      launcher: Launcher::new(config),
    })
    .invoke_handler(tauri::generate_handler![close, submit, search])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}