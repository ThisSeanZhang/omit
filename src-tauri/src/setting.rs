use std::fs::{self, OpenOptions};
use std::io::Write;

use log::debug;
use crate::{HomePath, error::IPCError, SETTINGS_FILE_NAME};



#[tauri::command]
pub fn read_setting(path: tauri::State<HomePath>) -> Result<String, IPCError> {
  let path = path.0.clone().join(SETTINGS_FILE_NAME);
  let file_contents = fs::read_to_string(path)?;
  Ok(file_contents)
}

#[tauri::command]
pub fn save_setting(path: tauri::State<HomePath>, setting: String) -> Result<(), IPCError> {
  let path = path.0.clone().join(SETTINGS_FILE_NAME);
  debug!("save_setting : {}", setting);
  let file = OpenOptions::new().create(true).write(true).truncate(true).open(&path)?;
  writeln!(&file, "{}", setting)?;

  Ok(())
}

