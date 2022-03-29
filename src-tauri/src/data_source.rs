use std::path::PathBuf;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

use crate::error::OmitError;

pub struct DataSource {
  data_path: PathBuf,
}

impl DataSource {
  pub fn new() -> DataSource {
    DataSource {
      data_path: std::env::current_dir().unwrap().clone().join("data.db"),
    }
  }

  pub fn get_conn(&self) -> Result<Connection, OmitError> {
    Connection::open(&self.data_path).map_err(OmitError::parse_error)
    // match Connection::open(path.join("cats.db")) {
    //     Ok(conn) => Ok(conn),
    //     Err(err) => OmitError::new(OmitErrorType::Default, format!("{}", e)),
    // }
  }
}

