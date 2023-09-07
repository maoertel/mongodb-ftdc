mod cli;
mod error;
mod model;
mod progress;
mod service;

use cli::Cli;
use service::{FtdcDataService, FtdcLoader};

use clap::Parser;
use error::Error;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let Cli {
    group_key,
    replica_set_name,
    size,
    public,
    private,
  } = Cli::parse();

  let service = FtdcDataService { client: Client::new() };

  service
    .get_ftdc_data(&group_key, &replica_set_name, size, &public, &private)
    .await
    .map(|download_path| println!("Downloaded to: `{download_path}`"))
}
