#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::error;
use error::Error;
use error::Error::InvalidJobState;

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogCollectionJob<'a> {
  resourceType: &'a str,
  resourceName: &'a str,
  redacted: bool,
  sizeRequestedPerFileBytes: u64,
  logTypes: Vec<&'a str>,
}

impl LogCollectionJob<'_> {
  pub(crate) fn from(replica_set_name: &str, bytes: u64) -> LogCollectionJob {
    LogCollectionJob {
      resourceName: replica_set_name,
      sizeRequestedPerFileBytes: bytes,
      resourceType: "REPLICASET",
      redacted: true,
      logTypes: vec!["FTDC"],
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clusters {
  pub results: Vec<Shard>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shard {
  pub(crate) userAlias: String,
  pub(crate) typeName: String,
  pub(crate) replicaSetName: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JobId {
  pub(crate) id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatus<'a> {
  pub(crate) id: &'a str,
  pub(crate) downloadUrl: &'a str,
  pub(crate) status: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JobState {
  SUCCESS,
  FAILURE,
  IN_PROGRESS,
  MARKED_FOR_EXPIRY,
  EXPIRED,
}

impl FromStr for JobState {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Error> {
    match s {
      "SUCCESS" => Ok(JobState::SUCCESS),
      "FAILURE" => Ok(JobState::FAILURE),
      "IN_PROGRESS" => Ok(JobState::IN_PROGRESS),
      "MARKED_FOR_EXPIRY" => Ok(JobState::MARKED_FOR_EXPIRY),
      "EXPIRED" => Ok(JobState::EXPIRED),
      invalid => Err(InvalidJobState(invalid.to_string())),
    }
  }
}
