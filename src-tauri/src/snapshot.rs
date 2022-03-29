use std::sync::{Arc, Mutex};
use std::{collections::HashMap, time::SystemTime};

use serde::{ Serialize, Deserialize};
use tauri::State;
use std::path::PathBuf;

use crate::common::ValueType;
use crate::config::OmitConfig;

pub struct SnapConfig {
  config_name: &'static str,
  base_path: PathBuf,
  snapshots: Vec<Snapshot>,
}

impl OmitConfig for SnapConfig {
    fn base_path(&self) -> PathBuf {
      self.base_path.clone()
    }
    fn auto_gen(&self) -> bool {
      true
    }
    fn config_name(&self) -> &'static str {
      self.config_name
    }
}

impl SnapConfig {
  pub fn new() -> SnapConfig {
    let mut config = SnapConfig {
      config_name: "snapshots.json",
      base_path: std::env::current_dir().unwrap().clone(),
      snapshots: vec![],
    };
    let snaps_raw = config.read();
    config.snapshots = snaps_raw.unwrap();
    // snaps.push(Snapshot::default());
    // config.save(&snaps);
    config
  }

  pub fn get_snaps(&self) -> &Vec<Snapshot>{
    &self.snapshots
  }

  pub fn get_snap_str(&self) -> String {
    if let Ok(result) = serde_json::to_string_pretty(&self.snapshots) {
      result
    } else {
      "".to_string()
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct SnapOption {
  pub brief_name: String,
  pub full_name: String,
  pub ignore: bool,
  pub option_type: ValueType,
  pub selected: bool,
  pub value: String,
}

impl Default for SnapOption {
  fn default() -> Self {
    SnapOption {
      brief_name: "".to_string(),
      full_name: "".to_string(),
      ignore: false,
      option_type: ValueType::STRING,
      selected: true,
      value: "".to_string(),
    }
  }
}
pub struct SnapConfigState(pub Arc<Mutex<SnapConfig>>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct SnapParam {
  pub selected: bool,
  pub param_type: ValueType,
  pub value: String,
}

impl Default for SnapParam {
  fn default() -> Self {
    SnapParam {
      selected: true,
      value: "".to_string(),
      param_type: ValueType::STRING,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Snapshot {
  snap_id: String,// 存储的是snap的id
  snap_type: String, // 快照类型 cmd commit
  ccid: String, // 如果是Commit的快照
  cid: String, // command id
  title: String,
  param_value: Vec<SnapParam>,
  option_value: Vec<SnapOption>,
  share: bool,
  // share_code: String,
  did: String, // 开发者ID
  version: String,
  command_name: String,
  create_time: u64,
  desc: HashMap<String, String>,
  location: String,
}
static SNAP_TABLE: &str = "create table if not exists snap_option (
  snap_id integer primary key,
  title text not null,
  create_time text not null,
  data BLOB not null
)";

// 先测试 所以就有默认值  之后需要删除
impl Default for Snapshot {
  fn default() -> Self {
    Snapshot {
      snap_id: "".to_string(),// 存储的是snap的id
      snap_type: "".to_string(), // 快照类型 cmd commit
      ccid: "".to_string(), // 如果是Commit的快照
      cid: "".to_string(), // command id
      title: "".to_string(),
      param_value: Vec::new(),
      option_value:Vec::new(),
      share: false,
      // share_code: String,
      did: "".to_string(), // 开发者ID
      version: "".to_string(),
      command_name: "".to_string(),
      create_time: SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_millis() as u64,
      desc: HashMap::new(),
      location: "".to_string(),
    }
  }
}

#[tauri::command]
pub fn read_snapshots(snap_config: State<'_, SnapConfig>) -> Result<String, String> {
  Ok(snap_config.read_str(snap_config.config_name()).unwrap_or("".to_string()))
}

pub fn test_snapshots() -> Result<Vec<Snapshot>, String> {
  let mut snap1 = Snapshot::default();
  snap1.title = "目录遍历".to_string();
  snap1.command_name = "ls".to_string();
  let mut op1 = SnapOption::default();
  op1.brief_name = "a".to_string();
  op1.full_name = "a".to_string();
  let mut op2 = SnapOption::default();
  op2.brief_name = "l".to_string();
  op2.full_name = "l".to_string();
  snap1.option_value.push(op1);
  snap1.option_value.push(op2);

  let mut snap2 = Snapshot::default();
  snap2.title = "SSH".to_string();
  snap2.command_name = "ssh".to_string();
  let mut op3 = SnapOption::default();
  op3.brief_name = "help".to_string();
  op3.full_name = "help".to_string();
  snap2.option_value.push(op3);

  let mut snap3 = Snapshot::default();
  snap3.title = "SFTP".to_string();
  snap3.command_name = "sftp".to_string();
  
  let mut snap4 = Snapshot::default();
  snap4.title = "SFTP".to_string();
  snap4.command_name = "sftp".to_string();
  
  let mut snap5 = Snapshot::default();
  snap5.title = "SFTP".to_string();
  snap5.command_name = "sftp".to_string();
  
  let mut snap6 = Snapshot::default();
  snap6.title = "SFTP".to_string();
  snap6.command_name = "sftp".to_string();
  
  let mut snap7 = Snapshot::default();
  snap7.title = "SFTP".to_string();
  snap7.command_name = "sftp".to_string();
  
  let mut snap8 = Snapshot::default();
  snap8.title = "SFTP".to_string();
  snap8.command_name = "sftp".to_string();
  
  let mut snap9 = Snapshot::default();
  snap9.title = "SFTP".to_string();
  snap9.command_name = "sftp".to_string();
  Ok(vec![snap1, snap2, snap3, snap4, snap5, snap6, snap7, snap8, snap9])
}