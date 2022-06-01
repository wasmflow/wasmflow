use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use vino_provider_cli::options::LatticeCliOptions;
use vino_provider_cli::LoggingOptions;

mod manifest;
mod wasm;

#[derive(Debug, Clone, Args)]
#[clap(rename_all = "kebab-case")]
pub(crate) struct TestCommand {
  #[clap(flatten)]
  pub(crate) logging: LoggingOptions,

  #[clap(flatten)]
  pub(crate) lattice: LatticeCliOptions,

  #[clap(flatten)]
  pub(crate) fetch: super::FetchOptions,

  #[clap(flatten)]
  wasi: crate::wasm::WasiOptions,

  /// Turn on info logging.
  #[clap(long = "info")]
  pub(crate) info: bool,

  /// Pass a seed along with the invocation.
  #[clap(long = "seed", short = 's', env = "VINO_SEED")]
  seed: Option<u64>,

  /// The path or OCI URL to a wafl manifest or wasm file.
  pub(crate) location: String,

  /// The test data.
  data_path: PathBuf,

  /// Filter which tests to run
  filter: Vec<String>,
}
#[allow(clippy::future_not_send, clippy::too_many_lines)]
pub(crate) async fn handle_command(opts: TestCommand) -> Result<()> {
  let _guard = logger::init(&opts.logging.name("vino"));

  let bytes = vino_loader::get_bytes(&opts.location, opts.fetch.allow_latest, &opts.fetch.insecure_registries)
    .await
    .context("Could not load from location")?;

  if crate::wasm::is_wasm(&bytes) {
    wasm::handle_command(opts, bytes).await
  } else {
    manifest::handle_command(opts, bytes).await
  }
}