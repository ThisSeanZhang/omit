use std::path::PathBuf;
use crate:: error::IPCError;
use crate::database::SaveUnit;

// 先默认存一下
const DEFAULT_SESSION_FILE_NAME: &str = "session-0";

#[tauri::command]
pub fn append_sesssions(path: String, sesssions: Vec<(String, String)>) -> Result<(), IPCError> {

  let sesssions_floder = PathBuf::from(path);

  if !sesssions_floder.exists() {
    std::fs::create_dir_all(&sesssions_floder)?;
  } else if sesssions_floder.is_file() {
    return Err(IPCError::SomeThingWrong("shortcut folder should a folder".to_owned()));
  }
  let note_path = sesssions_floder.join(DEFAULT_SESSION_FILE_NAME);
  SaveUnit::append(&note_path, sesssions)
}


#[tauri::command]
pub fn read_sesssions(path: String) -> Result<Vec<String>, IPCError> {
  let sesssions_floder = PathBuf::from(path).join(DEFAULT_SESSION_FILE_NAME);
  SaveUnit::list(&sesssions_floder)
}

#[tauri::command]
pub fn delete_sesssion(path: String, id: Option<String>) -> Result<(), IPCError> {
  let sesssions_floder = PathBuf::from(path).join(DEFAULT_SESSION_FILE_NAME);
  SaveUnit::delete(&sesssions_floder, id)
}
