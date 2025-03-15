use std::result;
use thiserror::Error;

use sqlx::Error as SqlxError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("[database] {0}")]
  DB(String),
  #[error("[internal] {0}")]
  Internal(String),
  #[error("[file-system] {0}")]
  FsError(String),
  #[error("[integrity] {0}")]
  Integrity(String),
  #[error("[command error] {}: {}",.bin, .message)]
  Command{bin: String, message: String },
}

impl From<SqlxError> for Error {
  fn from(e: SqlxError) -> Self {
    Error::DB(format!("{}", e))
  }
}

