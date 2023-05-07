use std::path::PathBuf;
use crate:: error::IPCError;
use crate::database::SaveUnit;

// 先默认存一下
const DEFAULT_COMMAND_FILE_NAME: &str = "command-0";

#[tauri::command]
pub fn append_commands(path: String, commands: Vec<(String, String)>) -> Result<(), IPCError> {

  let commands_floder = PathBuf::from(path);

  if !commands_floder.exists() {
    std::fs::create_dir_all(&commands_floder)?;
  } else if commands_floder.is_file() {
    return Err(IPCError::SomeThingWrong("command folder should a folder".to_owned()));
  }
  let note_path = commands_floder.join(DEFAULT_COMMAND_FILE_NAME);
  SaveUnit::append(&note_path, commands)
}


#[tauri::command]
pub fn read_commands(path: String) -> Result<Vec<String>, IPCError> {
  let commands_floder = PathBuf::from(path).join(DEFAULT_COMMAND_FILE_NAME);
  SaveUnit::list(&commands_floder)
}

#[tauri::command]
pub fn delete_command(path: String, id: Option<String>) -> Result<(), IPCError> {
  let commands_floder = PathBuf::from(path).join(DEFAULT_COMMAND_FILE_NAME);
  SaveUnit::delete(&commands_floder, id)
}
