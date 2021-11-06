use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
  DiqwestError(diqwest::error::Error),
  ReqwestError(reqwest::Error),
  JsonError(serde_json::Error),
  IoError(std::io::Error),
  InvalidJobState(String),
  DownloadError(String),
  CheckJobStatusError(String),
  CreateJobError(String),
  ReplicaSetNotFoundError(String),
  MongoJobError(String),
}

impl std::error::Error for Error {}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Error::DiqwestError(e) => std::fmt::Display::fmt(e, f),
      Error::ReqwestError(e) => std::fmt::Display::fmt(e, f),
      Error::JsonError(e) => std::fmt::Display::fmt(e, f),
      Error::IoError(e) => std::fmt::Display::fmt(e, f),
      Error::InvalidJobState(e) => std::fmt::Display::fmt(e, f),
      Error::DownloadError(e) => std::fmt::Display::fmt(e, f),
      Error::CheckJobStatusError(e) => std::fmt::Display::fmt(e, f),
      Error::CreateJobError(e) => std::fmt::Display::fmt(e, f),
      Error::ReplicaSetNotFoundError(e) => std::fmt::Display::fmt(e, f),
      Error::MongoJobError(e) => std::fmt::Display::fmt(e, f),
    }
  }
}

impl From<diqwest::error::Error> for Error {
  fn from(diqwest_error: diqwest::error::Error) -> Self {
    Error::DiqwestError(diqwest_error)
  }
}

impl From<reqwest::Error> for Error {
  fn from(reqwest_error: reqwest::Error) -> Self {
    Error::ReqwestError(reqwest_error)
  }
}

impl From<serde_json::Error> for Error {
  fn from(serde_json_error: serde_json::Error) -> Self {
    Error::JsonError(serde_json_error)
  }
}

impl From<std::io::Error> for Error {
  fn from(io_error: std::io::Error) -> Self {
    Error::IoError(io_error)
  }
}
