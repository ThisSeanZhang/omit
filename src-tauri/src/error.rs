pub enum ErrorType {
  Default,
}

pub struct Error {
  pub t: ErrorType,
  pub message: String,
}