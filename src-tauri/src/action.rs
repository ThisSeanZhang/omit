use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum ChannelAction {
    Message(String),
    SizeChange {
        width: i32,
        height: i32,
        width_px: Option<i32>,
        height_px: Option<i32>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Porter {
  pub data: Vec<u8>,
}