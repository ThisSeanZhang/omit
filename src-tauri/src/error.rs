#[derive(Debug)]
pub enum OmitErrorType {
  DEFAULT,
  CONFIG,
  RepositoryError,
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

impl OmitError {
  pub fn parse_error(err: impl std::error::Error) -> OmitError {
    OmitError {
      t: OmitErrorType::DEFAULT,
      message: err.to_string()
    }
  }
}