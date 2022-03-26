use std::{collections::HashMap, time::SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdReop {
  pub name: String,
  pub cmd_path: String,
  pub snap_path: String,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Snapshot {
  snap_id: String,// 存储的是snap的id
  snap_type: String, // 快照类型 cmd commit
  ccid: String, // 如果是Commit的快照
  cid: String, // command id
  title: String,
  param_value: Vec<CmdParam>,
  option_value: Vec<CmdOption>,
  share: bool,
  // share_code: String,
  did: String, // 开发者ID
  version: String,
  command_name: String,
  create_time: u64,
  desc: HashMap<String, String>,
  location: String,
}
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
pub fn read_snapshots() -> Result<Vec<Snapshot>, String> {
  let mut snap1 = Snapshot::default();
  snap1.title = "目录遍历".to_string();
  snap1.command_name = "ls".to_string();
  let mut snap2 = Snapshot::default();
  snap2.title = "SSH".to_string();
  snap2.command_name = "ssh".to_string();
  let mut snap3 = Snapshot::default();
  snap3.title = "SFTP".to_string();
  snap3.command_name = "sftp".to_string();
  Ok(vec![snap1, snap2, snap3])
}