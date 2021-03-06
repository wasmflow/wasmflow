use std::time::SystemTime;

use anyhow::Result;
use seeded_random::Seed;
use tokio::io::{self, AsyncBufReadExt};
use wasmflow_collection_cli::options::DefaultCliOptions;
use wasmflow_collection_cli::parse_args;
use wasmflow_host::HostBuilder;
use wasmflow_manifest::WasmflowManifest;
use wasmflow_sdk::v1::transport::TransportMap;

use crate::utils::merge_config;

pub(crate) async fn handle_command(opts: super::InvokeCommand, bytes: Vec<u8>) -> Result<()> {
  debug!(args = ?opts.args, "rest args");

  let manifest = WasmflowManifest::load_from_bytes(Some(opts.location), &bytes)?;

  let server_options = DefaultCliOptions {
    mesh: opts.mesh,
    ..Default::default()
  };

  let mut config = merge_config(&manifest, &opts.fetch, Some(server_options));
  if config.default_flow().is_none() {
    config.set_default_flow(opts.component);
  }

  let default_schematic = config.default_flow().clone().unwrap();

  let host_builder = HostBuilder::from_definition(config);

  let mut host = host_builder.build();
  host.connect_to_mesh().await?;
  host.start_network(opts.seed.map(Seed::unsafe_new)).await?;

  let signature = host.get_signature()?;
  let target_schematic = signature.get_component(&default_schematic);

  let mut check_stdin = !opts.no_input && opts.data.is_empty() && opts.args.is_empty();
  if let Some(target_schematic) = target_schematic {
    if target_schematic.inputs.is_empty() {
      check_stdin = false;
    }
  }

  let inherent_data = opts.seed.map(|seed| {
    wasmflow_sdk::v1::InherentData::new(
      seed,
      SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap(),
    )
  });

  if check_stdin {
    if atty::is(atty::Stream::Stdin) {
      eprintln!("No input passed, reading from <STDIN>. Pass --no-input to disable.");
    }
    let reader = io::BufReader::new(io::stdin());
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
      debug!("STDIN:'{}'", line);
      let mut payload = TransportMap::from_json_output(&line)?;
      if !opts.raw {
        payload.transpose_output_name();
      }

      let stream = host.request(&default_schematic, payload, inherent_data).await?;

      cli_common::functions::print_stream_json(stream, &opts.filter, opts.short, opts.raw).await?;
    }
  } else {
    let mut data_map = TransportMap::from_kv_json(&opts.data)?;

    let mut rest_arg_map = parse_args(&opts.args)?;
    if !opts.raw {
      data_map.transpose_output_name();
      rest_arg_map.transpose_output_name();
    }
    data_map.merge(rest_arg_map);

    let stream = host.request(&default_schematic, data_map, inherent_data).await?;
    cli_common::functions::print_stream_json(stream, &opts.filter, opts.short, opts.raw).await?;
  }
  host.stop().await;

  Ok(())
}
