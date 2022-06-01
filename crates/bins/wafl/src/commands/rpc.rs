use std::net::Ipv4Addr;
use std::path::PathBuf;

use clap::{Args, Subcommand};

pub(crate) mod invoke;
pub(crate) mod list;
pub(crate) mod stats;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum SubCommands {
  /// Invoke a component or schematic on a provider.
  #[clap(name = "invoke")]
  Invoke(invoke::Options),

  /// Query a provider for a list of its hosted components.
  #[clap(name = "list")]
  List(list::Options),

  /// Query a provider for its runtime statistics.
  #[clap(name = "stats")]
  Stats(stats::Options),
}

#[derive(Debug, Clone, Args)]
pub(crate) struct ConnectOptions {
  /// RPC port.
  #[clap(short, long, env = vino_provider_cli::options::env::VINO_RPC_PORT)]
  pub(crate) port: u16,

  /// RPC address.
  #[clap(short, long, default_value = "127.0.0.1", env = vino_provider_cli::options::env::VINO_RPC_ADDRESS)]
  pub(crate) address: Ipv4Addr,

  /// Path to pem file for TLS connections.
  #[clap(long)]
  pub(crate) pem: Option<PathBuf>,

  /// Path to client key for TLS connections.
  #[clap(long)]
  pub(crate) key: Option<PathBuf>,

  /// Path to CA pem for TLS connections.
  #[clap(long)]
  pub(crate) ca: Option<PathBuf>,

  /// The domain to verify against the certificate.
  #[clap(long)]
  pub(crate) domain: Option<String>,
}
