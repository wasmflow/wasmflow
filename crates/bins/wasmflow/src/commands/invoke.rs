use anyhow::{Context, Result};
use clap::Args;
use vino_provider_cli::options::LatticeCliOptions;

mod manifest;
mod wasm;

#[derive(Debug, Clone, Args)]
#[clap(rename_all = "kebab-case")]
pub(crate) struct InvokeCommand {
  #[clap(flatten)]
  pub(crate) logging: super::LoggingOptions,

  #[clap(flatten)]
  pub(crate) lattice: LatticeCliOptions,

  #[clap(flatten)]
  wasi: crate::wasm::WasiOptions,

  #[clap(flatten)]
  pub(crate) fetch: super::FetchOptions,

  /// Turn on info logging.
  #[clap(long = "info")]
  pub(crate) info: bool,

  /// Path or OCI url to manifest or wasm file.
  location: String,

  // *****************************************************************
  // Everything below is copied from common-cli-options::RunOptions
  // Flatten doesn't work with positional args...
  //
  // TODO: Eliminate the need for copy/pasting
  // *****************************************************************
  /// Name of the component to execute.
  #[clap(default_value = "default")]
  component: String,

  /// Don't read input from STDIN.
  #[clap(long = "no-input")]
  no_input: bool,

  /// Skip additional I/O processing done for CLI usage.
  #[clap(long = "raw", short = 'r')]
  raw: bool,

  /// Filter the outputs by port name.
  #[clap(long = "filter")]
  filter: Vec<String>,

  /// A port=value string where value is JSON to pass as input.
  #[clap(long = "data", short = 'd')]
  data: Vec<String>,

  /// Print values only and exit with an error code and string on any errors.
  #[clap(long = "values", short = 'o')]
  short: bool,

  /// Pass a seed along with the invocation.
  #[clap(long = "seed", short = 's', env = "VINO_SEED")]
  seed: Option<u64>,

  /// Arguments to pass as inputs to a schematic.
  #[clap(last(true))]
  args: Vec<String>,
}

pub(crate) async fn handle_command(mut opts: InvokeCommand) -> Result<()> {
  let mut logging = &mut opts.logging;
  if !(opts.info || logging.trace || logging.debug) {
    logging.quiet = true;
  }
  let _guard = logger::init(&logging.name("vino"));

  let bytes = vino_loader::get_bytes(&opts.location, opts.fetch.allow_latest, &opts.fetch.insecure_registries)
    .await
    .context("Could not load from location")?;

  if crate::wasm::is_wasm(&bytes) {
    wasm::handle_command(opts, bytes).await
  } else {
    manifest::handle_command(opts, bytes).await
  }
}
