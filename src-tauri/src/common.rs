use serde::{ Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ValueType {
  NONE = 0, NUMBER = 1, ENUM = 2, STRING = 3, PAIR = 4,
}