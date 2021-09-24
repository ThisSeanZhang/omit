#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod ssh_cmd;
mod config;
mod util;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use config::{ ConfigState, Config, save_session, sessions, read_session};
use std::env;
use ssh_cmd::{create_ssh, new_window, add_listen, current_path, SSHState};

fn main() {
  let config = Config::init();
  println!("config file: {:?}", config);
  tauri::Builder::default()
  .manage(SSHState(Arc::new(Mutex::new(HashMap::new()))))
  .manage(ConfigState(Arc::new(Mutex::new(config))))
  .invoke_handler(tauri::generate_handler![
    send_data_from,
    create_ssh,
    new_window,
    add_listen,
    current_path,
    save_session,
    sessions,
    read_session,
  ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

#[tauri::command]
fn send_data_from() {
  println!("I was invoked from JS!");
}