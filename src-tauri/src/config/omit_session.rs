use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OmitSession {
  pub name: String,
  ip: String,
  port: u16,
  username: String,
  passwd: String,
}