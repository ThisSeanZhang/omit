#[derive(Debug)]
pub enum OmitErrorType {
  Default,
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