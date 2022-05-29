# Download mongodb FTDC data

Lib crate to enable your application to download FTDC data from mongodb clusters to investigate deeper (e.g. [keyhole](https://github.com/simagix/keyhole)).

## Usage

To download FTDC data from a mongodb cluster you need to provide some input to the CLI:

1. **Group key:** The group key (or: project id) the respective cluster belongs too. It is encoded into the link you get from atlas when selecting the specific cluster on Atlas UI (e.g.`cloud.mongodb.com/v2/{group key}clusters`)
2. **Replicaset name:** The name of the replica set the data should be loaded from. You can either provide the direct targeted replica set name (e.g. `atlas-<something>-shard-0`) or the name of the shard (e.g `some-name-shard-00`). Do not forget the number here as it qualifies the shard in case you want data from a sharded cluster. For a standalone replica set it is`00` but lets say for a sharded cluster with 3 shards it would be `00`, `01`, `02`.
3. **API key:**  You need to have a valid API key for at least the cluster you want to download FTDC data from.

```rust
use error::Error;
use reqwest::Client;
use service::{FtdcDataService, FtdcLoader};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let group_key = "...";
  let replica_set_name = "...";
  let size = 10_000_000
  let public = "...";
  let private = "...";

  let service = FtdcDataService { client: Client::new() };

  service
    .get_ftdc_data(
      group_key,
      replica_set_name,
      size,
      public,
      private,
    )
    .await
    .map(|download_path| println!("Downloaded to: `{download_path}`"))
}
```

The data is downloaded to the current directory the application was executed in as a `*.tar.gz`.
