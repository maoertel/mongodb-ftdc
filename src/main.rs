mod error;
mod model;
mod progress;
mod service;

use clap::Parser;
use error::Error;
use reqwest::Client;
use service::{FtdcDataService, FtdcLoader};

/// Loading FTDC data (full time diagnostic data capture) from a particular db cluster to
/// investigate deeper (e.g. with keyhole).
#[derive(Parser)]
#[clap(version = "0.1.1", author = "Mathias Oertel <mathias.oertel@pm.me>")]
struct Opts {
  /// The group key (or: project id) the respective cluster belongs too. It is encoded into
  /// the link you get from atlas when selecting the specific cluster on Atlas UI (e.g.
  /// `cloud.mongodb.com/v2/{group key}#clusters`).
  #[clap(long, short)]
  group_key: String,
  /// The name of the replica set the data should be loaded from. You can either provide the
  /// direct targeted replica set name (e.g. `atlas-<something>-shard-0`) or the name of the
  /// shard (e.g. `some-name-shard-00`). Do not forget the number here as it qualifies the
  /// shard in case you want data from a sharded cluster. For a standalone replica set it is
  ///`00` but lets say for a sharded cluster with 3 shards it would be `00`, `01`, `02`.
  #[clap(long, short)]
  replica_set_name: String,
  /// Optional byte size of the downloaded job. If the data in your job is not going back in
  /// time enough: increase the byte size. [default: 10.000.000]
  #[clap(long, short)]
  size: Option<u64>,
  /// The public key of your Atlas API key.
  #[clap(long)]
  public: String,
  /// The private key of your Atlas API key.
  #[clap(long)]
  private: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let opts: Opts = Opts::parse();

  let service = FtdcDataService { client: Client::new() };

  service
    .get_ftdc_data(
      &opts.group_key,
      &opts.replica_set_name,
      opts.size.unwrap_or(10_000_000),
      &opts.public,
      &opts.private,
    )
    .await
    .map(|download_path| println!("Downloaded to: `{}`", download_path))
}
