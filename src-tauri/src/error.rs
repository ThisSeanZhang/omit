use thiserror::Error;
use std::io;

use tauri::InvokeError;

#[derive(Debug)]
pub enum OmitErrorType {
  Default,
  ReadFile,
  SaveError,
  CONFIG,
  RepositoryError,
  CommandError,
}

#[derive(Debug)]
pub struct OmitError {
  pub t: OmitErrorType,
  pub message: String,
}

impl OmitError {
  pub fn new(t: OmitErrorType, message: String) -> OmitError {
    OmitError {
      t,
      message,
    }
  }
}

#[derive(Error, Debug)]
pub enum APPError {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    
    #[error("{0}")]
    PathError(String),
}

#[derive(Error, Debug)]
pub enum IPCError {
    #[error("failed to operator: {0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    SomeThingWrong(String),

    #[error("{0}")]
    Serde(#[from] serde_json::Error),

}

impl Into<InvokeError> for IPCError {
  fn into(self) -> InvokeError {
      InvokeError::from(self.to_string())
  }
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, APPError>;
