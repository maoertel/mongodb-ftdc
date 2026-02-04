use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

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
    pub fn from(replica_set_name: &str, bytes: u64) -> LogCollectionJob<'_> {
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
    pub user_alias: String,
    pub type_name: String,
    pub replica_set_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JobId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus<'a> {
    pub id: &'a str,
    pub download_url: &'a str,
    pub status: &'a str,
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
