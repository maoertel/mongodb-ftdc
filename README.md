# Download mongodb FTDC data

Command line tool and lib crate to download Full Time Diagnostic Data Capture (FTDC) data from mongodb clusters to investigate with e.g. [keyhole](https://github.com/simagix/keyhole).

## Crate

For the documentation of the lib crate functionality go [here](/README_LIB.md).

## CLI tool to download mongodb FTDC data

Command line tool to download FTDC data from mongodb clusters.

### Usage

To download FTDC data from a mongodb cluster you need to provide some input to the CLI:

1. **Group key:** The group key (or: project id) the respective cluster belongs too. It is encoded into the link you get from atlas when selecting the specific cluster on Atlas UI (e.g.`cloud.mongodb.com/v2/{group key}/clusters`)
2. **Replicaset name:** The name of the replica set the data should be loaded from. You can either provide the direct targeted replica set name (e.g. `atlas-<something>-shard-0`) or the name of the shard (e.g `some-name-shard-00`). Do not forget the number here as it qualifies the shard in case you want data from a sharded cluster. For a standalone replica set it is `00` but lets say for a sharded cluster with 3 shards it would be `00`, `01`, `02`.
3. **API key:** You need to have a valid API key for at least the cluster you want to download FTDC data from. The credential can be provided by parameters (`--atlas-public-key`, `--atlas-private-key`) or environment variables (`ATLAS_PUBLIC_KEY`, `ATLAS_PRIVATE_KEY`)

```bash
ftdc --group-key <group key> \
  --replica-set-name <rs name> \
  --atlas-public-key <public key> \
  --atlas-private-key <private key>
```

The data is downloaded to the current directory as a `*.tar.gz` file.

### Installation

#### Brew

Install with brew for macOS (amd4/arm64) or Linux (amd64).

```bash
brew tap maoertel/tap
brew install ftdc
```

#### Cargo install

If you have `cargo` installed just execute the following command to install `ftdc` on your machine.

```bash
cargo install ftdc
```

#### Build yourself

Check this repo out and execute the following command:

```bash
cargo build --bin ftdc --features "build-binary" --release
```

#### Download the binaries

You can download binaries for macOS (amd4/arm64) or Linux (amd64) from the github [release page](https://github.com/maoertel/mongodb-ftdc/releases).

## License

[MIT](./MIT-LICENSE)
