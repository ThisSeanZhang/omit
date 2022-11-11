use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct FrontEndPorter {
  pub action: FrontEndAction,
  pub data: Option<Vec<u8>>,
  pub message: Option<String>,
}

impl FrontEndPorter {
  pub fn shell_data(data: Vec<u8>) -> FrontEndPorter {
    FrontEndPorter {
      action: FrontEndAction::Message,
      data: Some(data),
      message: None,
    }
  }

  pub fn action(action: FrontEndAction, message: Option<String>) -> FrontEndPorter {
    FrontEndPorter {
      action,
      data: None,
      message,
    }
  }
}
#[derive(Serialize, Deserialize, Clone)]
pub enum FrontEndAction {
    Message,
    Eof,
}