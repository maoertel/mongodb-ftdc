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
    let Cli { group_key, replica_set_name, size, atlas_public_key, atlas_private_key } =
        Cli::parse();

    let service = FtdcDataService::new(Client::new());

    service
        .get_ftdc_data(
            &group_key,
            &replica_set_name,
            size,
            &atlas_public_key,
            &atlas_private_key,
        )
        .await
        .map(|download_path| println!("Downloaded to: `{download_path}`"))
}
