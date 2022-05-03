use std::path::PathBuf;

use tauri::State;

use crate::{config::ConfigState, util};

const SHORTCUT_FILE_NAME: &'static str = "_shortcuts.json";

#[tauri::command]
pub fn read_shortcuts(config: State<ConfigState>) -> Result<String, String> {
  // Ok(snap_config.read_str(snap_config.config_name()).unwrap_or("".to_string()))
  let path = if let Ok(conf) = config.0.lock() {
    conf.repos_folder.clone()
  } else {
    return Err("get config error".to_string());
  };
  let mut path = PathBuf::from(path);
  match util::read_file(&path, SHORTCUT_FILE_NAME) {
    Ok(data) => Ok(data),
    Err(e) => Err(e.message),
  }
}

#[tauri::command]
pub fn save_shortcuts(config: State<ConfigState>, snaps: String) -> Result<String, String> {
  let path = if let Ok(conf) = config.0.lock() {
    conf.repos_folder.clone()
  } else {
    return Err("get config error".to_string());
  };
  match util::save_file(&PathBuf::from(path), SHORTCUT_FILE_NAME, &snaps) {
    Ok(_) => Ok("success".to_string()),
    Err(e) => Err(e.message),
  }
}