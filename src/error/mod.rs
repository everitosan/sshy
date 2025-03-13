use std::result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("[file-system] {0}")]
  FsError(String),
  #[error("[integrity] {0}")]
  Integrity(String),
  #[error("[command error] {}: {}",.bin, .message)]
  Command{bin: String, message: String },
}

pub type Result<T> = result::Result<T, Error>;
