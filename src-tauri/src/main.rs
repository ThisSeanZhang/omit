#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod action;
mod shell;
mod config;
// mod terminal;
mod error;
mod snapshot;
mod shortcut;
mod common;
mod repository;
mod command;
// mod ssh_cmd;
mod util;

use shell::create_pty;
use config::{ ConfigState, Config, save_session, sessions, read_session};
use snapshot::{read_snapshots, save_snapshots, SnapConfig};
use shortcut::{read_shortcuts, save_shortcuts};
use crate::repository::{
  repository_state_init,
  get_commands,
  get_repo_dirs,
  read_repo_command,
};
// use ssh_cmd::{create_ssh, add_listen, current_path, SSHState};
use std::{collections::HashMap, sync::{Arc, Mutex}};

fn main() {
  let config = Config::init();
  println!("config file: {:?}", config);

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      create_pty,
      read_shortcuts,
      save_shortcuts,
      read_repo_command,
      save_snapshots,
      get_repo_dirs,
      get_commands,
      create_pty,
      // send_data_from,
      // create_ssh,
      // new_window,
      // add_listen,
      // current_path,
      save_session,
      sessions,
      read_session,
      read_snapshots,
    ])
    .manage(SnapConfig::new())
    // .manage(SSHState(Arc::new(Mutex::new(HashMap::new()))))
    .manage(ConfigState(Arc::new(Mutex::new(config))))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}