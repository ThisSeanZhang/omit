mod omit_session;

use serde::{Deserialize, Serialize};
use std::{convert::TryInto, env, fs::{self, OpenOptions}};
use std::io::{Error, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::fs::File;
use tauri::{AppHandle, State, Window, command};

use crate::util;

use self::omit_session::OmitSession;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub session_fload: String,
}
pub struct ConfigState(pub Arc<Mutex<Config>>);

fn default_config() -> Config {
  let path = env::current_dir().unwrap().clone();
  Config {
    session_fload: path.join("session").as_path().to_str().unwrap().to_string(),
  }
}

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
  fn new(config_json: Option<String>) ->  Config {
    let mut default = default_config();
    if let Some(json) = config_json {
      print!("read config json: {}", json);
      let read_config = serde_json::from_str(&json);
      if read_config.is_err() {
        panic!("config not match");
      }
      return read_config.unwrap();
    }
    default.save();
    default
  }

  pub fn init() -> Config {
    let runtime_dir = env::current_dir().unwrap();
    let file = util::read_raw_json(&env::current_dir().unwrap(), "config.json");
    let config = Config::new(file);
    check_fload_and_create(&config.session_fload);
    config
  }

  fn save(&self) -> Result<String, String>{
    let config_path = env::current_dir().unwrap().clone().join("config.json");
    let file = OpenOptions::new().write(true).create(true).open(config_path);
    let json_config = serde_json::to_string_pretty(&self);
    if json_config.is_err() {
      return Err(format!("save error reason: {}", json_config.unwrap_err()));
    }
    file.unwrap().write(json_config.unwrap().as_bytes());
    Ok("save_sucess".into())
  }
}

#[command(async)]
pub fn save_session(app: AppHandle, config:State<ConfigState>, sess:OmitSession) -> impl std::future::Future<Output = Result<String, String>> {
  let session_fload =  config.0.lock().unwrap().session_fload.clone();
  let mut file_path = PathBuf::from(session_fload).join(sess.name.clone());
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
  let session_fload =  config.0.lock().unwrap().session_fload.clone();
  let dirs = PathBuf::from(session_fload).read_dir();
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
  let session_fload =  config.0.lock().unwrap().session_fload.clone();
  let sess_json = util::read_raw_json(&PathBuf::from(session_fload), format!("{}.json",session_name).as_str());
  if sess_json.is_none() {
    return Err("load session error".into());
  }
  let session = serde_json::from_str(sess_json.unwrap().as_str());
  if session.is_err() {
    return Err("load session error".into());
  }
  Ok(session.unwrap())
}