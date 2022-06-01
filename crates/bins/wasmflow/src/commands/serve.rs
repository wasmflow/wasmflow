use anyhow::{Context, Result};
use clap::Args;
use vino_provider_cli::options::DefaultCliOptions;
mod manifest;
mod wasm;

#[derive(Debug, Clone, Args)]
pub(crate) struct ServeCommand {
  #[clap(flatten)]
  pub(crate) cli: DefaultCliOptions,

  #[clap(flatten)]
  pub(crate) fetch: super::FetchOptions,

  #[clap(flatten)]
  wasi: crate::wasm::WasiOptions,

  /// The path or OCI URL to a wafl manifest or wasm file.
  pub(crate) location: String,
}

pub(crate) async fn handle_command(opts: ServeCommand) -> Result<()> {
  let _guard = logger::init(&opts.cli.logging.name("vino"));

  let bytes = vino_loader::get_bytes(&opts.location, opts.fetch.allow_latest, &opts.fetch.insecure_registries)
    .await
    .context("Could not load from location")?;

  if crate::wasm::is_wasm(&bytes) {
    wasm::handle_command(opts, bytes).await
  } else {
    manifest::handle_command(opts, bytes).await
  }
}
