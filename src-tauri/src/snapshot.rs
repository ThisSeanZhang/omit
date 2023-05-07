use std::path::PathBuf;
use crate:: error::IPCError;
use crate::database::SaveUnit;

// 先默认存一下
const DEFAULT_SNAPSHOT_FILE_NAME: &str = "snapshot-0";

#[tauri::command]
pub fn append_snapshots(path: String, snapshots: Vec<(String, String)>) -> Result<(), IPCError> {

  let snapshots_floder = PathBuf::from(path);

  if !snapshots_floder.exists() {
    std::fs::create_dir_all(&snapshots_floder)?;
  } else if snapshots_floder.is_file() {
    return Err(IPCError::SomeThingWrong("snapshot folder should a folder".to_owned()));
  }
  let note_path = snapshots_floder.join(DEFAULT_SNAPSHOT_FILE_NAME);
  SaveUnit::append(&note_path, snapshots)
}


#[tauri::command]
pub fn read_snapshots(path: String) -> Result<Vec<String>, IPCError> {
  let snapshots_floder = PathBuf::from(path).join(DEFAULT_SNAPSHOT_FILE_NAME);
  SaveUnit::list(&snapshots_floder)
}

#[tauri::command]
pub fn delete_snapshot(path: String, id: Option<String>) -> Result<(), IPCError> {
  let snapshots_floder = PathBuf::from(path).join(DEFAULT_SNAPSHOT_FILE_NAME);
  SaveUnit::delete(&snapshots_floder, id)
}
