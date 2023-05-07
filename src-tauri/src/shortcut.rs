use std::path::PathBuf;
use crate:: error::IPCError;
use crate::database::SaveUnit;

// 先默认存一下
const DEFAULT_SHORTCUT_FILE_NAME: &str = "shortcuts-0";

#[tauri::command]
pub fn append_shortcuts(path: String, shortcuts: Vec<(String, String)>) -> Result<(), IPCError> {

  let shortcuts_floder = PathBuf::from(path);

  if !shortcuts_floder.exists() {
    std::fs::create_dir_all(&shortcuts_floder)?;
  } else if shortcuts_floder.is_file() {
    return Err(IPCError::SomeThingWrong("shortcut folder should a folder".to_owned()));
  }
  let note_path = shortcuts_floder.join(DEFAULT_SHORTCUT_FILE_NAME);
  SaveUnit::append(&note_path, shortcuts)
}


#[tauri::command]
pub fn read_shortcuts(path: String) -> Result<Vec<String>, IPCError> {
  let shortcuts_floder = PathBuf::from(path).join(DEFAULT_SHORTCUT_FILE_NAME);
  SaveUnit::list(&shortcuts_floder)
}

#[tauri::command]
pub fn delete_shortcut(path: String, id: Option<String>) -> Result<(), IPCError> {
  let shortcuts_floder = PathBuf::from(path).join(DEFAULT_SHORTCUT_FILE_NAME);
  SaveUnit::delete(&shortcuts_floder, id)
}
