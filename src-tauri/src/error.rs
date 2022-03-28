#[derive(Debug)]
pub enum OmitErrorType {
  Default,
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