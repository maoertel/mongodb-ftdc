use std::env;
use std::fs::File;
use std::io;
use std::str::FromStr;
use std::thread;

use async_recursion::async_recursion;
use async_trait::async_trait;
use diqwest::core::WithDigestAuth;
use indicatif::ProgressBar;
use reqwest::{Client, StatusCode};
use std::time;

use crate::error;
use crate::model;
use crate::progress;

use error::Error;
use model::{Clusters, JobId, JobState, JobStatus, LogCollectionJob, Shard};
use progress::SpinnerHelper;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const MONGODB_URL: &str = "https://cloud.mongodb.com/api/atlas/v1.0/groups";

fn url() -> String {
  #[cfg(not(test))]
  let url = String::from(MONGODB_URL);
  #[cfg(test)]
  let url = mockito::server_url();
  url
}

#[async_trait]
pub trait FtdcLoader {
  async fn get_ftdc_data(
    &self,
    group_key: &str,
    replica_set_name: &str,
    byte_size: u64,
    public: &str,
    private: &str,
  ) -> Result<String, Error>;
}

pub struct FtdcDataService {
  pub client: Client,
}

#[async_trait]
impl FtdcLoader for FtdcDataService {
  async fn get_ftdc_data(
    &self,
    group_key: &str,
    replica_set_name: &str,
    byte_size: u64,
    public: &str,
    private: &str,
  ) -> Result<String, Error> {
    let replica_set = self.get_replica_set(group_key, replica_set_name, public, private).await?;
    let job_id = self
      .create_ftdc_job(group_key, &replica_set, byte_size, public, private)
      .await?;

    let check_job_status_spinner = SpinnerHelper::create(format!("Check job status of job with id: {}", &job_id.id));
    let _download_url = self
      .check_job_state(group_key, &job_id.id, &check_job_status_spinner, public, private)
      .await?;

    let download_ftdc_data_spinner =
      SpinnerHelper::create(format!("Start to download FTDC data for job with id: {}", &job_id.id));

    self
      .download_ftdc_data(
        group_key,
        &job_id.id,
        &replica_set,
        &download_ftdc_data_spinner,
        public,
        private,
      )
      .await
  }
}

impl FtdcDataService {
  async fn get_replica_set(
    &self,
    group_key: &str,
    replica_set_name: &str,
    public: &str,
    private: &str,
  ) -> Result<String, Error> {
    let processes = self
      .client
      .get(&format!("{}/{}/processes", url(), group_key))
      .send_with_digest_auth(public, private)
      .await?;

    match processes.status() {
      StatusCode::OK => {
        let response_body = processes.text().await?;
        let shards = serde_json::from_str::<Clusters>(&response_body)?.results;
        let shards: Vec<Shard> = shards
          .into_iter()
          .filter(|s| {
            s.replicaSetName.is_some()
              && (s.userAlias.contains(replica_set_name) || s.replicaSetName == Some(replica_set_name.to_string()))
          })
          .collect();

        shards.first().and_then(|s| s.replicaSetName.as_ref()).iter().fold(
          Err(Error::ReplicaSetNotFoundError(format!(
            "No replica set found that corresponds to {}",
            replica_set_name
          ))),
          |_, s| Ok(s.to_string()),
        )
      }
      _ => Err(Error::ReplicaSetNotFoundError(format!(
        "Something went wrong trying to get the list of running processes. Please try later. Currently running processes: {}",
        processes.text().await?
      ))),
    }
  }

  async fn create_ftdc_job(
    &self,
    group_key: &str,
    replica_set: &str,
    byte_size: u64,
    public: &str,
    private: &str,
  ) -> Result<JobId, Error> {
    println!("Starting FTDC data job for ReplicaSet: {}", replica_set);

    let body = serde_json::to_string::<LogCollectionJob>(&LogCollectionJob::from(replica_set, byte_size))?;
    let create_ftdc_job = self
      .client
      .post(format!("{}/{}/logCollectionJobs", url(), group_key))
      .header("Content-type", "application/json; charset=utf-8")
      .body(body)
      .send_with_digest_auth(public, private)
      .await?;

    match create_ftdc_job.status() {
      StatusCode::CREATED => {
        let response_body = create_ftdc_job.text().await?;
        Ok(serde_json::from_str::<JobId>(&response_body)?)
      }
      _ => Err(Error::CreateJobError(format!(
        "Something went wrong  creating the FTDC job. {}",
        create_ftdc_job.text().await?
      ))),
    }
  }

  #[async_recursion]
  async fn check_job_state(
    &self,
    group_key: &str,
    job_id: &str,
    spinner: &ProgressBar,
    public: &str,
    private: &str,
  ) -> Result<String, Error> {
    let check_job_status = self
      .client
      .get(&format!("{}/{}/logCollectionJobs/{}", url(), group_key, job_id))
      .send_with_digest_auth(public, private)
      .await?;

    match check_job_status.status() {
      StatusCode::OK => {
        let job_status = check_job_status.text().await?;
        let job_status = serde_json::from_str::<JobStatus>(&job_status)?;

        match JobState::from_str(job_status.status)? {
          JobState::IN_PROGRESS => {
            spinner.set_message(format!("IN_PROGRESS – job id: {}", job_id));
            thread::sleep(time::Duration::from_millis(3000));
            self.check_job_state(group_key, job_id, spinner, public, private).await
          }
          JobState::SUCCESS | JobState::MARKED_FOR_EXPIRY => {
            spinner.finish_with_message(format!("SUCCESS – FTDC data for job with id {} will be downloaded.", job_id));
            Ok(String::from(job_status.downloadUrl))
          }
          JobState::FAILURE | JobState::EXPIRED => {
            spinner.abandon_with_message(format!("FAILURE – Something went wrong creating job with id {}.", job_id));
            Err(Error::MongoJobError(
              "Failure while job creation. Please try again.".to_string(),
            ))
          }
        }
      }
      _ => Err(Error::CheckJobStatusError(format!(
        "Something went wrong checking the jobs status. Try again later. Error message: {}",
        check_job_status.text().await?
      ))),
    }
  }

  async fn download_ftdc_data(
    &self,
    group_key: &str,
    job_id: &str,
    replica_set: &str,
    spinner: &ProgressBar,
    public: &str,
    private: &str,
  ) -> Result<String, Error> {
    let download_url = format!("{}/{}/logCollectionJobs/{}/download", url(), group_key, job_id);
    let response = self.client.get(&download_url).send_with_digest_auth(public, private).await?;

    match response.status() {
      StatusCode::OK => {
        spinner.set_message(format!("PROGRESS – Download FTDC data for job with id: {}", job_id));

        let bytes = response.bytes().await?;
        let mut slice: &[u8] = bytes.as_ref();
        let file_name = format!("ftdc_data_{}_job_{}.tar.gz", replica_set, job_id);
        let file_name = file_name.as_str();
        let mut out = File::create(file_name)?;
        io::copy(&mut slice, &mut out)?;

        spinner.finish_with_message(format!("SUCCESS – FTDC data for job with id {} downloaded.", job_id));

        Ok(format!("{}/{}", env::current_dir()?.display(), file_name))
      }
      _ => Err(Error::DownloadError(format!(
        "Something went wrong downloading the FTDC data. Try to download at: {}. Status code: {}. Body: {}",
        download_url,
        response.status(),
        response.text().await?
      ))),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::model::{Clusters, JobId, JobStatus, Shard};
  use crate::service::FtdcDataService;
  use indicatif::ProgressBar;
  use mockito::mock;
  use reqwest::Client;

  fn ftdc_data_service() -> FtdcDataService {
    FtdcDataService { client: Client::new() }
  }

  #[tokio::test]
  async fn given_explicit_replica_set_name_when_get_replica_set_then_get_the_same_name() {
    // Given
    let clusters = Clusters {
      results: vec![Shard {
        userAlias: "something that does not matter".to_string(),
        typeName: "".to_string(),
        replicaSetName: Some("my-replica-set".to_string()),
      }],
    };
    let _m = mock("GET", "/my-group-key/processes")
      .with_status(200)
      .with_header("content-type", "application/json; charset=utf-8")
      .with_body(serde_json::to_string(&clusters).unwrap())
      .create();

    // When
    let response = ftdc_data_service()
      .get_replica_set("my-group-key", "my-replica-set", "", "")
      .await
      .unwrap();

    // Then
    assert_eq!(&response, "my-replica-set");
  }

  #[tokio::test]
  async fn given_shard_name_when_get_replica_set_then_get_corresponding_replica_set() {
    // Given
    let clusters = Clusters {
      results: vec![Shard {
        userAlias: "my-rs-shard-00".to_string(),
        typeName: "".to_string(),
        replicaSetName: Some("my-replica-set".to_string()),
      }],
    };
    let _m = mock("GET", "/my-group-key/processes")
      .with_status(200)
      .with_header("content-type", "application/json; charset=utf-8")
      .with_body(serde_json::to_string(&clusters).unwrap())
      .create();

    // When
    let response = ftdc_data_service()
      .get_replica_set("my-group-key", "my-rs-shard-00", "", "")
      .await
      .unwrap();

    // Then
    assert_eq!(&response, "my-replica-set");
  }

  #[tokio::test]
  async fn given_wrong_name_when_get_replica_set_then_no_rs_error() {
    // Given
    let clusters = Clusters {
      results: vec![Shard {
        userAlias: "my-rs-shard-00".to_string(),
        typeName: "".to_string(),
        replicaSetName: Some("my-replica-set".to_string()),
      }],
    };
    let _m = mock("GET", "/my-group-key/processes")
      .with_status(200)
      .with_header("content-type", "application/json; charset=utf-8")
      .with_body(serde_json::to_string(&clusters).unwrap())
      .create();

    // When
    let replica_set_not_found_error = ftdc_data_service()
      .get_replica_set("my-group-key", "another-rs-shard-00", "", "")
      .await
      .unwrap_err()
      .to_string();

    // Then
    assert_eq!(
      replica_set_not_found_error,
      "No replica set found that corresponds to another-rs-shard-00"
    );
  }

  #[tokio::test]
  async fn given_no_processes_in_the_given_group_when_get_replica_set_then_no_rs_error() {
    // Given
    let clusters = Clusters { results: vec![] };
    let _m = mock("GET", "/my-group-key/processes")
      .with_status(200)
      .with_header("content-type", "application/json; charset=utf-8")
      .with_body(serde_json::to_string(&clusters).unwrap())
      .create();

    // When
    let replica_set_not_found_error = ftdc_data_service()
      .get_replica_set("my-group-key", "another-rs-shard-00", "", "")
      .await
      .unwrap_err()
      .to_string();

    // Then
    assert_eq!(
      replica_set_not_found_error,
      "No replica set found that corresponds to another-rs-shard-00"
    );
  }

  #[tokio::test]
  async fn given_replica_set_when_create_ftdc_job_then_give_job_id() {
    // Given
    let job_id = JobId {
      id: String::from("new-job-id-73"),
    };
    let _m = mock("POST", "/my-group-key/logCollectionJobs")
      .with_status(201)
      .with_header("content-type", "application/json; charset=utf-8")
      .with_body(serde_json::to_string(&job_id).unwrap())
      .create();

    // When
    let response = ftdc_data_service()
      .create_ftdc_job("my-group-key", "another-rs-shard-00", 10, "", "")
      .await
      .unwrap();

    // Then
    assert_eq!(
      response,
      JobId {
        id: String::from("new-job-id-73")
      }
    );
  }

  #[tokio::test]
  async fn given_job_id_when_check_job_state_then_give_success() {
    // Given
    let job_id = "new-job-id-73";
    let job_status = JobStatus {
      id: "any id",
      downloadUrl: "download from here",
      status: "SUCCESS",
    };
    let spinner = ProgressBar::new_spinner();
    let _m = mock("GET", "/my-group-key/logCollectionJobs/new-job-id-73")
      .with_status(200)
      .with_header("content-type", "application/json; charset=utf-8")
      .with_body(serde_json::to_string(&job_status).unwrap())
      .create();

    // When
    let response = ftdc_data_service()
      .check_job_state("my-group-key", job_id, &spinner, "", "")
      .await
      .unwrap();

    // Then
    assert_eq!(response, String::from("download from here"));
  }
}
