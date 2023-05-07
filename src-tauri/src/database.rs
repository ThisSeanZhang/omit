
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{Write, BufReader, BufRead};

use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::error::IPCError;

#[derive(Serialize, Deserialize)]
pub enum SaveUnit {
  Date(String, String),
  Del(String)
}

// pub struct LogFileDataBase {
  
// }
impl SaveUnit {

  pub fn list(units_path: &PathBuf) -> Result<Vec<String>, IPCError> {

    if units_path.is_dir() {
      return Err(IPCError::SomeThingWrong("target path is a dir".to_string()));
    }
    
    if !units_path.exists() {
      return Ok(vec![]);
    }

    let file = OpenOptions::new()
      .read(true)
      .truncate(false)
      .open(units_path)?;

  let reader = BufReader::new(file);

  // let mut lines = reader.lines();

  // let mut all_sentences  = std::collections::VecDeque::new();
  
  // let mut delete_id = HashSet::new();

  // all valid key
  let mut key_map: HashMap<String, (usize, usize, String, String)> = HashMap::new();

  // junk data
  let mut junk_data_num: usize = 0;
  // 读取剩下的行
  for (index, line) in reader.lines().enumerate() {
    let line = line?;
    // debug!("line len is: {}", line.len());
    let old = match serde_json::from_str(&line) {
      Ok(SaveUnit::Date(id, data)) => {
        let old = key_map.remove(&id);
        // 四个值 分别是 在文件中的顺序 数据段大小 唯一识别ID 数据自身
        let insert_data = if let Some((old_index, ..)) = &old {
          (old_index.clone(), line.len(), id.clone(), data)
        } else {
          (index, line.len(), id.clone(), data)
        };
        key_map.insert(id, insert_data);
        old
      },
      Ok(SaveUnit::Del(id)) => {
        junk_data_num += line.len();
        key_map.remove(&id)
      },
      Err(e) => return Err(IPCError::SomeThingWrong(format!("serde_json parse error: {:?}", e))),
    };
    if let Some((_index, line_len, _id, _data)) = old {
      junk_data_num += line_len;
      // debug!("junk data mun is : {junk_data_num}");
    }
  }

  let mut values = key_map.into_values().collect::<Vec<(usize, usize, String, String)>>();
  values.sort_by(|(index_a, ..), (index_b, ..)| index_a.cmp(index_b));
  // debug!("junk data mun is : {junk_data_num}");
  if junk_data_num > 1024 {
    info!("file: {:?}, junk data more then 1024 clean it", units_path.file_stem());
    let file = OpenOptions::new().write(true).truncate(true).open(&units_path)?;
    for (.., id,  data) in values.iter() {
      let obj = SaveUnit::Date(id.clone(), data.clone());
      writeln!(&file, "{}", serde_json::to_string(&obj)?)?;
    }
  }
  // println!("{values:?}");
  Ok(values.into_iter().map(|(.., data)| data).collect())
  }

  pub fn append(units_path: &PathBuf, units: Vec<(String, String)>) -> Result<(), IPCError> {
    if units_path.is_dir() {
      return Err(IPCError::SomeThingWrong("it should not a dir".to_owned()));
    }
    let file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(units_path)?;
  
   
    for (each_id, each) in units {
      let obj = SaveUnit::Date(each_id, each);
      writeln!(&file, "{}", serde_json::to_string(&obj)?)?;
    }

    Ok(())
  }

  pub fn delete(units_path: &PathBuf, id: Option<String>) -> Result<(), IPCError> {
    if let Some(id) = id {
      debug!("delete file: {units_path:?}, id: {id:}");
      let file = OpenOptions::new()
        .append(true)
        .open(units_path)?;
  
        let obj = SaveUnit::Del(id);
        writeln!(&file, "{}", serde_json::to_string(&obj)?)?;
    } else {
      debug!("clean file");
      // Truncate the file
      OpenOptions::new().write(true).truncate(true).open(&units_path)?;
    }
  
    Ok(())
  }

}
