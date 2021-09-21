#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod ssh_cmd;
use std::{collections::HashMap, sync::{Arc, Mutex}};

use ssh_cmd::{create_ssh, new_window, add_listen, current_path, SSHState};

fn main() {
  tauri::Builder::default()
  .manage(SSHState(Arc::new(Mutex::new(HashMap::new()))))
  .invoke_handler(tauri::generate_handler![
    send_data_from,
    create_ssh,
    new_window,
    add_listen,
    current_path,
  ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

#[tauri::command]
fn send_data_from() {
  println!("I was invoked from JS!");
}