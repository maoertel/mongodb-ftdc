use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::error::Error::InvalidJobState;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionJob<'a> {
    resource_type: &'a str,
    resource_name: &'a str,
    redacted: bool,
    size_requested_per_file_bytes: u64,
    log_types: Vec<&'a str>,
}

impl LogCollectionJob<'_> {
    pub(crate) fn from(replica_set_name: &str, bytes: u64) -> LogCollectionJob {
        LogCollectionJob {
            resource_name: replica_set_name,
            size_requested_per_file_bytes: bytes,
            resource_type: "REPLICASET",
            redacted: true,
            log_types: vec!["FTDC"],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clusters {
    pub results: Vec<Shard>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shard {
    pub(crate) user_alias: String,
    pub(crate) type_name: String,
    pub(crate) replica_set_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JobId {
    pub(crate) id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus<'a> {
    pub(crate) id: &'a str,
    pub(crate) download_url: &'a str,
    pub(crate) status: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JobState {
    Succcess,
    Failure,
    InProgress,
    MarkedForExpiry,
    Expired,
}

impl FromStr for JobState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "SUCCESS" => Ok(JobState::Succcess),
            "FAILURE" => Ok(JobState::Failure),
            "IN_PROGRESS" => Ok(JobState::InProgress),
            "MARKED_FOR_EXPIRY" => Ok(JobState::MarkedForExpiry),
            "EXPIRED" => Ok(JobState::Expired),
            invalid => Err(InvalidJobState(invalid.to_string())),
        }
    }
}
