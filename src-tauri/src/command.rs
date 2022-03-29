use std::{collections::HashMap, time::SystemTime, path::PathBuf};

use serde::{Deserialize, Serialize};
use crate::config::OmitConfig;

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

pub struct Command {
  cid: String,
  command_name: String,
  brief_desc: HashMap<String, String>,
  description: HashMap<String, String>,
  version: String,
  platform: String,
  arg_num: u8,
  frequency: u32,
  options: Vec<CmdOption>,
  params: Vec<CmdParam>
}

#[derive(Serialize, Deserialize, Debug)]
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
  fn default() -> Self{
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


