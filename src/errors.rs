
use std::{io, string};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("I/O error")]
  IoError(#[from] io::Error),

  #[error("Validation error")]
  ValidationError(String),

  #[error("Utf error")]
  FromUtfError(#[from] string::FromUtf8Error),

  #[error ("Serde error")]
  SerdeError(#[from] serde_json::Error)
}
