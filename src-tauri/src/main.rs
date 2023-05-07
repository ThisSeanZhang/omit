#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use std::{fs, path::PathBuf};
use tauri::api::path::home_dir;
use tauri_plugin_log::LogTarget;

const APP_FOLDER: &str = ".omit";
const SETTINGS_FILE_NAME: &str = "settings.json";

pub struct HomePath(pub PathBuf);

mod database;
mod setting;
mod session;
mod snapshot;
mod common;

mod action;
mod shell;
mod check_list;
mod config;
// mod terminal;
mod error;
mod shortcut;
// mod repository;
mod command;
// mod ssh_cmd;
mod util;

use error::{Result, APPError};
use setting::{
  read_setting,
  save_setting,
};
use shortcut::{
  append_shortcuts,
  read_shortcuts,
  delete_shortcut,
};
use session::{
  append_sesssions,
  read_sesssions,
  delete_sesssion,
};
use snapshot::{
  append_snapshots,
  read_snapshots,
  delete_snapshot,
};
use command::{
  append_commands,
  read_commands,
  delete_command,
};

use shell::create_pty;

fn main() -> Result<()> {
  let home = home_dir().map(|path| path.join(APP_FOLDER)).ok_or(APPError::PathError("can not get home path".to_owned()))?;
  if !home.exists() {
    fs::create_dir_all(&home)?;
  } else if home.is_file() {
    // TODO: New Windows and notice user
    return Err(APPError::PathError(".nobody exists and is a file please delete it".to_owned()));
  }
  
  // let config = Config::init();
  // println!("config file: {:?}", config);

  tauri::Builder::default()
      .plugin(tauri_plugin_log::Builder::default()
      .level(log::LevelFilter::Debug)
      .targets([
        LogTarget::LogDir,
        LogTarget::Stdout,
        LogTarget::Webview,
      ]).build()
    )
    .manage(HomePath(home.clone()))
    .invoke_handler(tauri::generate_handler![
      // setting
      read_setting,
      save_setting,
      // shortcuts
      append_shortcuts,
      read_shortcuts,
      delete_shortcut,
      //session
      append_sesssions,
      read_sesssions,
      delete_sesssion,
      // snapshots
      append_snapshots,
      read_snapshots,
      delete_snapshot,
      // command
      append_commands,
      read_commands,
      delete_command,
      // pty
      create_pty,
      // exists,
      // send_data_from,
      // create_ssh,
      // new_window,
      // add_listen,
      // current_path,
    ])
    // .manage(SSHState(Arc::new(Mutex::new(HashMap::new()))))
    // .manage(ConfigState(Arc::new(Mutex::new(config))))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  Ok(())
}