use std::fs;
use std::path::PathBuf;
use std::ffi::OsString;
use serde::{Serialize, Deserialize};
use serde_json::{ self, Error, Result};

#[derive(PartialEq)]
pub enum OmitFileType {
  DIR,
  FILE,
}
pub struct OmitFileInfo {
  pub file_name: String,
  pub file_path: PathBuf,
  file_type: OmitFileType,
}

impl OmitFileInfo {
    pub fn is_dir(&self) -> bool {
      self.file_type == OmitFileType::DIR
    }

    pub fn is_file(&self) -> bool {
      self.file_type == OmitFileType::FILE
    }

    pub fn is_hide(&self) -> bool {
      self.file_name.starts_with('.')
      // if let Some(file_name) = self.file_name.to_str() {
      //   file_name.starts_with('.')
      // } else {
      //   false
      // }
    }

    pub fn is_extend(&self, extend: &str) -> bool {
      self.file_name.ends_with(&format!(".{}", extend))
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

pub fn list_dir(path: &PathBuf) -> Vec<OmitFileInfo> {
  println!("read path: {:?}", path);
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
        files.push(OmitFileInfo {
          file_name: path.file_name().to_str().unwrap().to_string(),
          file_path,
          file_type
        })
      }
    }
  }
  files
}