use std::fs;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use serde_json::{ self, Error, Result};

pub fn read_raw_json(path: &PathBuf, file_name: &str) -> Option<String> {
  let mut file_path = path.clone();
  file_path.push(file_name);
  if !file_path.is_file() {
    return None;
  }
  fs::read_to_string(file_path.as_path().clone())
  .map_or_else(|_| None, |json| Some(json))
}