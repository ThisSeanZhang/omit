use std::ffi::OsString;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Mutex, Arc};
use std::{collections::HashMap, time::SystemTime};

use serde::{ Serialize, Deserialize};
use tauri::{AppHandle, State};

use crate::command::Command;
use crate::config::{Config, self, ConfigState};
use crate::error::{ OmitError, OmitErrorType };
use crate::util::{self, OmitFileInfo};

pub struct RepositoryState(pub Arc<Mutex<Vec<Repository>>>);

pub fn repository_state_init(config: &Config) -> RepositoryState {
  RepositoryState(Arc::new(Mutex::new(deal_all_repository(&config.repos_folder))))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository{
  pub name: String,
  pub path: PathBuf,
  pub commands: Vec<Command>,
}

impl Repository {
  // pub fn new(repo_config_json: Option<String>) -> Result<Repository, OmitError> {
  //   if let Some(json) = repo_config_json {
  //     print!("read config json: {}", json);
  //     let repo_config = serde_json::from_str(&json);
  //     // let result:Result<HashMap<String, String>, Error> = serde_json::from_str(&json);
  //     if repo_config.is_err() {
  //       return Err(OmitError::new(
  //         OmitErrorType::RepositoryError,
  //         format!("convert repo error, str: {}", json)
  //       ));
  //     }
  //     return Ok(repo_config.unwrap());
  //   }
  //   Err(OmitError::new(
  //     OmitErrorType::RepositoryError,
  //     format!("repo config is Empty")
  //   ))
  // }

  pub fn new(dir_info: OmitFileInfo) -> Result<Repository, OmitError>{
    Ok(Repository{
      name: dir_info.file_name,
      path: dir_info.file_path,
      commands: vec![],
    })
  }

  fn scan_commands(&mut self) {
    for info in util::list_dir(&self.path) {
      if info.is_file() && !info.is_hide() {
        let file = util::read_raw_json(&self.path, &info.file_name);
        if let Ok(cmds) = Command::new(file) {
          self.commands.extend(cmds);
        }
      }
    }
  }
}

pub fn deal_all_repository(path: &str) -> Vec<Repository> {
  let mut repos = vec![];
  for info in util::list_dir(&path.into()) {
    if info.is_dir() {
      // let file = util::read_raw_json(&info.file_path, "repository.json");
      if let Ok(mut repo) = Repository::new(info) {
        repo.scan_commands();
        repos.push(repo);
      }
    }
  }
  repos
}



#[tauri::command(async)]
pub fn get_commands(app: AppHandle, repos:State<RepositoryState>) -> impl std::future::Future<Output = Result<String, String>> {
  let result = if let Ok(data) = repos.0.lock() {
    if let Ok(data_json) = serde_json::to_string_pretty(Deref::deref(&data)) {
      Ok(data_json)
    } else {
      Err("error".into())
    }
  } else {
    Err("error".into())
  };
  std::future::ready(result)
}

#[tauri::command(async)]
pub fn get_repo_dirs(app: AppHandle, config_status:State<ConfigState>) -> impl std::future::Future<Output = Result<Vec<String>, String>> {
  let result = if let Ok(config) = config_status.0.lock() {
    let mut dirs = vec![];
    for info in util::list_dir(&PathBuf::from(&config.repos_folder)) {
      if info.is_dir() {
        dirs.push(info.file_name);
      }
    }
    Ok(dirs)
  } else {
    Err("error".into())
  };
  std::future::ready(result)
}


#[tauri::command(async)]
pub fn read_repo_command(app: AppHandle, config_status:State<ConfigState>, repo_dir: String ) -> impl std::future::Future<Output = Result<HashMap<String, String>, String>> {
  let result = if let Ok(config) = config_status.0.lock() {
    let mut command_map = HashMap::new();
    let mut path = PathBuf::from(&config.repos_folder);
    path.push(repo_dir);
    for info in util::list_dir(&path) {
      if info.is_hide() || !info.is_extend("json") { continue; }
      let data = util::read_raw_json(&path, &info.file_name).unwrap_or("[]".to_string());
      command_map.insert(info.file_name, data);
    }
    Ok(command_map)
  } else {
    Err("error".into())
  };
  std::future::ready(result)
}