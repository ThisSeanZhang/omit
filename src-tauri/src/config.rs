mod omit_session;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{convert::TryInto, env, fs::{self, OpenOptions}, collections::HashMap};
use std::io::{Error, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::fs::File;
use tauri::{AppHandle, State, Window, command};

use crate::util;
use crate::error::{ OmitError, OmitErrorType};

use self::omit_session::OmitSession;

pub enum ConfigType {
  MAIN,
  SNAPSHOT,
}

pub trait OmitConfig{
    fn config_name(&self) -> &'static str;
    fn base_path(&self) -> PathBuf;
    fn auto_gen(&self) -> bool {
      false
    }
    fn read<T>(&self) -> Result<T, OmitError>
    where T : Serialize + DeserializeOwned + Default {
      self.read_config(self.config_name())
    }
    fn read_config<T>(&self, config_name: &str) -> Result<T, OmitError>
    where T : Serialize + DeserializeOwned + Default {
      let config_json = self.read_str(config_name);
      if let Some(json) = config_json {
        print!("read config json: {}", json);
        let read_config = serde_json::from_str(&json);
        // let result:Result<HashMap<String, String>, Error> = serde_json::from_str(&json);
        if read_config.is_err() {
          return Err(OmitError::new(OmitErrorType::CONFIG, format!("json convert error: {}", read_config.err().unwrap())));
        }
        return Ok(read_config.unwrap());
      }
      if self.auto_gen() {
        return Ok(T::default());
      }
      Err(OmitError::new(OmitErrorType::CONFIG, format!("json read error")))
    }

    fn save<T>(&self, config: &T) -> Result<(), OmitError> where T: Serialize {
      self.save_config(self.config_name(), config)
    }

    fn save_config<T>(&self, config_name: &str, config: &T) -> Result<(), OmitError> where T: Serialize {
      let config_path = self.base_path().join(config_name);
      let file = OpenOptions::new().write(true).create(true).open(config_path);
      let json_config = serde_json::to_string_pretty(config);
      if json_config.is_err() {
        return Err(OmitError::new(
          OmitErrorType::CONFIG,
          format!("save error reason: {}", json_config.unwrap_err())
        ));
      }
      file.unwrap().write(json_config.unwrap().as_bytes());
      Ok(())
    }

    fn read_str(&self, config_name: &str) -> Option<String> {
      let mut file_path = self.base_path().join(config_name);
      if !file_path.is_file() {
        return None;
      }
      fs::read_to_string(file_path.as_path())
      .map_or_else(|_| None, |json| Some(json))
    }
  
    fn create_dir(&self, dir_name: &str) -> Result<(), OmitError> {
      let file_path = self.base_path().join(dir_name);
      if !file_path.exists() {
        let result = fs::create_dir_all(&file_path);
        if let Err(e) = result {
          return Err(OmitError::new(OmitErrorType::CONFIG, format!("create dir: {} error: {}", dir_name, e)));
        }
      }
      if !file_path.is_dir() {
        return Err(OmitError::new(OmitErrorType::CONFIG, format!("specified path not folder")));
      }
      Ok(())
    }
}

#[derive(Clone)]
pub struct ConfigManager {
  config_name: &'static str,
  base_path: PathBuf,
}

impl OmitConfig for ConfigManager {
  fn base_path(&self) -> PathBuf {
    self.base_path.clone()
  }

  fn config_name(&self) -> &'static str {
    self.config_name
  }
}

impl ConfigManager {
  pub fn new() -> ConfigManager {
    // TODO 环境变量读取替换
    ConfigManager {
      config_name: "config.json",
      base_path: env::current_dir().unwrap().clone()
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
  pub session_folder: String,
  pub repos_folder: String,
}

impl Default for Config {
  fn default() -> Self { 
    let path = env::current_dir().unwrap().clone();
    Config {
      session_folder: path.join("session").as_path().to_str().unwrap().to_string(),
      repos_folder: path.join("repos").as_path().to_str().unwrap().to_string(),
    }
  }
}

pub struct ConfigState(pub Arc<Mutex<Config>>);

fn check_fload_and_create(path:&String) {
  let path = PathBuf::from(path);
  if !path.exists() {
    let result = fs::create_dir_all(&path);
    if let Err(e) = result {
      panic!("create fold error: {}", e);
    }
  }

  if !path.is_dir() {
    panic!("specified path not folder");
  }
}

impl Config {
  pub fn init() -> Config {
    let config_manager = ConfigManager::new();
    let raw_config = config_manager.read_config("config.json");
    let config = if raw_config.is_err() {
      Config::default()
    } else {
      raw_config.unwrap()
    };
    // config_manager.save(&config);
    config
  }
}

#[command(async)]
pub fn save_session(app: AppHandle, config:State<ConfigState>, sess:OmitSession) -> impl std::future::Future<Output = Result<String, String>> {
  let session_folder =  config.0.lock().unwrap().session_folder.clone();
  let mut file_path = PathBuf::from(session_folder).join(sess.name.clone());
  file_path.set_extension("json");
  println!("file path: {:?}", file_path);
  let file = File::create(file_path);
  if let Err(e) = file {
    return std::future::ready(Err(e.to_string()));
  }
  let json_data = serde_json::to_string_pretty(&sess);
  if let Err(e) = json_data {
    return std::future::ready(Err(e.to_string()));
  }
  file.unwrap().write_all(json_data.unwrap().as_bytes());
  std::future::ready(Ok("success".into()))
}

#[command(async)]
pub fn sessions(app: AppHandle, config:State<ConfigState>) -> impl std::future::Future<Output = Result<Vec<String>, String>> {
  let session_folder =  config.0.lock().unwrap().session_folder.clone();
  let dirs = PathBuf::from(session_folder).read_dir();
  if let Err(e) = dirs {
    return std::future::ready(Err(e.to_string()));
  }
  let sess = dirs.ok().unwrap()
    .filter(|read_dir| read_dir.is_ok())
    .map(|e|e.unwrap())
    .filter(|e|e.path().exists())
    .map(|entry| entry.path().file_stem().unwrap().to_str().unwrap().to_string()).collect::<Vec<String>>();
    // .filter(|e| e.is_some()).map(|e| e.unwrap().to_string()).collect::<Vec<String>>();
    std::future::ready(Ok(sess))
}
#[command]
pub fn read_session(config:State<ConfigState>, session_name: String) -> Result<OmitSession, String> {
  let session_folder =  config.0.lock().unwrap().session_folder.clone();
  let sess_json = util::read_raw_json(&PathBuf::from(session_folder), format!("{}.json",session_name).as_str());
  if sess_json.is_none() {
    return Err("load session error".into());
  }
  let session = serde_json::from_str(sess_json.unwrap().as_str());
  if session.is_err() {
    return Err("load session error".into());
  }
  Ok(session.unwrap())
}

#[command]
pub fn exists(dir_path: &str, file_name: Option<&str>) -> Result<bool, String> {
  let mut path = PathBuf::from(dir_path);
  if let Some(file) = file_name {
    path.push(file);
  }
  if path.exists() {
    return Ok(true)
  } else {
    return Ok(false)
  }
}

#[command]
pub fn create_file(dir_path: &str, file_name: Option<&str>, data:Option<&str>) -> Result<(), String> {
  let mut path = PathBuf::from(dir_path);
  if !path.exists() {
    if let Err(e) = fs::create_dir_all(&path) {
      return Err("path create error".into());
    }
  }
  if let Some(file_name) = file_name {
    path.push(file_name);
    let file = File::create(path);
    if let Some(file_data) = data {
      match file {
          Ok(mut file) => {
            file.write_all(file_data.as_bytes()).map_err(|e| e.to_string())?;
          },
          Err(e) => {
            return Err(e.to_string())
          }
      }
    }
  }
  Ok(())
}

#[command]
pub fn read_file(file_path: &str) -> Result<String, String> {
  let file_path = PathBuf::from(file_path);
  if !file_path.is_file() {
    println!("read_file_path: {:?}", file_path);
    return Err("file don't exists".into());
  }
  fs::read_to_string(file_path).map_err(|e| e.to_string())
}

#[command]
pub fn list_dir_all(dir_path: &str) -> Result<Vec<String>, String> {
  let dir_path = PathBuf::from(dir_path);
  if dir_path.is_file() {
    return Err("This is a file path not a folder".into());
  }
  let dirs = dir_path.read_dir().map_err(|e| e.to_string())?;
  let result = dirs.filter_map(|path| path.ok())
  // .map(|entry| {
  //   println!("file name: {:?}", entry.file_name());
  //   entry.path().file_stem().unwrap().to_str().unwrap().to_string()
  // })
  .map(|entry| entry.file_name().to_string_lossy().to_string())
  .collect::<Vec<String>>();
  Ok(result)
}

#[command]
pub fn list_dir_only_folder(dir_path: &str) -> Result<Vec<String>, String> {
  let dir_path = PathBuf::from(dir_path);
  if dir_path.is_file() {
    return Err("This is a file path not a folder".into());
  }
  let dirs = dir_path.read_dir().map_err(|e| e.to_string())?;
  let result = dirs.filter_map(|path| path.ok())
  .filter(|entry| entry.file_type().unwrap().is_dir())
  .map(|entry| entry.file_name().to_string_lossy().to_string())
  .collect::<Vec<String>>();
  Ok(result)
}