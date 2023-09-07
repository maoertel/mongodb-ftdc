use clap::Parser;

/// Loading FTDC data (full time diagnostic data capture) from a particular replica set or dedicated
/// shard of a sharded clutser to investigate deeper (e.g. with keyhole).
#[derive(Parser)]
pub(crate) struct Cli {
  /// The group key (or: project id) the respective cluster belongs too. It is encoded into
  /// the link you get from atlas when selecting the specific cluster on Atlas UI (e.g.
  /// `cloud.mongodb.com/v2/{group key}#clusters`).
  #[clap(long, short)]
  pub(crate) group_key: String,
  /// The name of the replica set the data should be loaded from. You can either provide the
  /// direct targeted replica set name (e.g. `atlas-<something>-shard-0`) or the name of the
  /// shard (e.g. `some-name-shard-00`). Do not forget the number here as it qualifies the
  /// shard in case you want data from a sharded cluster. For a standalone replica set it is
  ///`00` but lets say for a sharded cluster with 3 shards it would be `00`, `01`, `02`.
  #[clap(long, short)]
  pub(crate) replica_set_name: String,
  /// Optional byte size of the downloaded job. If the data in your job is not going back in
  /// time enough: increase the byte size. [default: 10.000.000]
  #[clap(long, short, default_value = "10000000")]
  pub(crate) size: u64,
  /// The public key of your Atlas API key.
  #[clap(long)]
  pub(crate) public: String,
  /// The private key of your Atlas API key.
  #[clap(long)]
  pub(crate) private: String,
}
