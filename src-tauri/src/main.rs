#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod config;
mod terminal;
mod error;
mod action;
mod snapshot;
mod common;
mod repository;
mod command;

use snapshot::{read_snapshots, save_snapshots, SnapConfig};
use terminal::create_pty;
mod ssh_cmd;
mod util;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use config::{ ConfigState, Config, save_session, sessions, read_session};
// use std::env;
use ssh_cmd::{create_ssh, add_listen, current_path, SSHState};

use crate::repository::{
  repository_state_init,
  get_commands,
  get_repo_dirs,
  read_repo_command,
};
// use tauri::Manager;

fn main() {
  let config = Config::init();
  println!("config file: {:?}", config);
  tauri::Builder::default()
  // .setup(|app| {
  //   let window = app.get_window("main").unwrap();
  //   use tauri_plugin_vibrancy::Vibrancy;
  //   #[cfg(target_os = "windows")]
  //   window.apply_blur();
  //   #[cfg(target_os = "macos")]
  //   {
  //       use tauri_plugin_vibrancy::MacOSVibrancy;
  //       window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
  //   }
  //   Ok(())
  // })
  // .manage(repository_state_init(&config))
  .manage(SnapConfig::new())
  .manage(SSHState(Arc::new(Mutex::new(HashMap::new()))))
  .manage(ConfigState(Arc::new(Mutex::new(config))))
  .invoke_handler(tauri::generate_handler![
    read_repo_command,
    save_snapshots,
    get_repo_dirs,
    get_commands,
    create_pty,
    send_data_from,
    create_ssh,
    // new_window,
    add_listen,
    current_path,
    save_session,
    sessions,
    read_session,
    read_snapshots,
  ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

#[tauri::command]
fn send_data_from() {
  println!("I was invoked from JS!");
}