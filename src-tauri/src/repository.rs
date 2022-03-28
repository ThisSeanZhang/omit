use std::{collections::HashMap, time::SystemTime};

use serde::{ Serialize, Deserialize};

use crate::error::{ OmitError, OmitErrorType };
use crate::util;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository{
  pub name: String,
  pub path: String,
  pub command_name: Vec<String>,
}

impl Repository {
  pub fn new(repo_config_json: Option<String>) -> Result<Repository, OmitError> {
    if let Some(json) = repo_config_json {
      print!("read config json: {}", json);
      let repo_config = serde_json::from_str(&json);
      // let result:Result<HashMap<String, String>, Error> = serde_json::from_str(&json);
      if repo_config.is_err() {
        return Err(OmitError::new(
          OmitErrorType::RepositoryError,
          format!("convert repo error, str: {}", json)
        ));
      }
      return Ok(repo_config.unwrap());
    }
    Err(OmitError::new(
      OmitErrorType::RepositoryError,
      format!("repo config is Empty")
    ))
  }
}

fn deal_all_repository(path: String) -> Vec<Repository> {
  let mut repos = vec![];
  for info in util::list_dir(path) {
    if info.is_dir() {
      let file = util::read_raw_json(&info.file_path, "repository.json");
      if let Ok(repo) = Repository::new(file) {
        repos.push(repo);
      }
    }
  }
  repos
}



