use std::fs;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use serde_json::{ self, Error, Result};

#[derive(PartialEq)]
pub enum OmitFileType {
  DIR,
  FILE,
}
pub struct OmitFileInfo {
  pub file_path: PathBuf,
  file_type: OmitFileType,
}

impl OmitFileInfo {
    pub fn is_dir(&self) -> bool {
      self.file_type == OmitFileType::DIR
    }
}

pub fn read_raw_json(path: &PathBuf, file_name: &str) -> Option<String> {
  let mut file_path = path.clone();
  file_path.push(file_name);
  if !file_path.is_file() {
    return None;
  }
  fs::read_to_string(file_path.as_path().clone())
  .map_or_else(|_| None, |json| Some(json))
}

pub fn list_dir(path: String) -> Vec<OmitFileInfo> {
  let paths = fs::read_dir(path).unwrap();
  let mut files = vec![];
  for path_raw in paths {
    if let Ok(path) = path_raw {
      let file_path = path.path();
      if let Ok(metadata) = path.metadata() {
        let file_type = if metadata.is_dir() {
          OmitFileType::DIR
        } else {
          OmitFileType::FILE
        };
        files.push(OmitFileInfo { file_path, file_type })
      }
    }
  }
  files
}