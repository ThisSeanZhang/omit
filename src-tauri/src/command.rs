use std::{collections::HashMap, time::SystemTime, path::PathBuf};

use serde::{Deserialize, Serialize};
use crate::{config::OmitConfig, error::{OmitError, OmitErrorType}};

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdReop {
  pub name: String,
  pub cmd_path: String,
  pub snap_path: String,
}

pub struct RepoConfig {
  config_name: &'static str,
  base_path: PathBuf,
}

impl OmitConfig for RepoConfig {
  fn base_path(&self) -> PathBuf {
    self.base_path.clone()
  }
  fn auto_gen(&self) -> bool {
    false
  }
  fn config_name(&self) -> &'static str {
    self.config_name
  }
}

impl RepoConfig {
  pub fn new(repo_dir: &str) -> RepoConfig {
    let mut config = RepoConfig {
      config_name: "repository.json",
      base_path: std::env::current_dir().unwrap().clone().join("data"),
    };
    // snaps.push(Snapshot::default());
    // config.save(&snaps);
    config
  }
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Command {
  // cid: String,
  command_name: String,
  brief_desc: HashMap<String, String>,
  description: HashMap<String, String>,
  // version: String,
  platform: u8,
  // arg_num: u8,
  // frequency: u32,
  options: Vec<CmdOption>,
  params: Vec<CmdParam>
}

impl Default for Command {
  fn default() -> Self {
      Command {
        // cid: "".to_string(),
        command_name: "".to_string(),
        brief_desc: HashMap::new(),
        description: HashMap::new(),
        // version: "".to_string(),
        platform: 0,
        // arg_num: 0,
        // frequency: 0,
        options: vec![],
        params: vec![]
      }
  }
}

impl Command {
  pub fn new(cmd_json: Option<String>) -> Result<Vec<Command>, OmitError> {
    if let Some(json) = cmd_json {
      print!("read cmd json: {}", json);
      let repo_config = serde_json::from_str(&json);
      // let result:Result<HashMap<String, String>, Error> = serde_json::from_str(&json);
      if repo_config.is_err() {
        return Err(OmitError::new(
          OmitErrorType::CommandError,
          format!("convert cmd error, str: {}", json)
        ));
      }
      return Ok(repo_config.unwrap());
  }
    Err(OmitError::new(
      OmitErrorType::CommandError,
      format!("repo config is Empty")
    ))
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CmdOption {
  oid: String,
  cid: String,
  brief_name: String,
  full_name: String,
  description: HashMap<String, String>,
  frequency: u32,
  data_type: String,
  rules: Vec<String>,
  value: Vec<String>,
  // 使用的时候是否忽略类型
  ignore: bool,
  // 是否能重复选择
  duplicate: bool,
  selected: bool,
}

impl Default for CmdOption {
  fn default() -> Self {
    return CmdOption{
      oid: "".to_string().to_string(),
      cid: "".to_string(),
      brief_name: "".to_string(),
      full_name: "".to_string(),
      description: HashMap::new(),
      frequency: 0,
      data_type: "".to_string(),
      rules: Vec::new(),
      value: Vec::new(),
      // 使用的时候是否忽略类型
      ignore: false,
      // 是否能重复选择
      duplicate: false,
      selected: true,
    };
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdParam {
  cpid: String,
  cid: String,
  sort: u16,
  param_name: String,
  description: HashMap<String, String>,
  required: bool,
  param_type: String,
  value: String,
}


