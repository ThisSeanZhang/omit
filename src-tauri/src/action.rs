use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum ChannelAction {
    Message(String),
    SizeChange {
        width: u32,
        height: u32,
        width_px: Option<i32>,
        height_px: Option<i32>
    },
    EXIT,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Porter {
  pub data: Vec<u8>,
}