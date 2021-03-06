use anyhow::Result;
use seeded_random::Seed;
use wasmflow_collection_cli::options::DefaultCliOptions;
use wasmflow_host::HostBuilder;
use wasmflow_manifest::WasmflowManifest;

use crate::utils::merge_config;

pub(crate) async fn handle_command(opts: super::RunCommand, bytes: Vec<u8>) -> Result<()> {
  debug!(args = ?opts.args, "rest args");
  let manifest = WasmflowManifest::load_from_bytes(Some(opts.location.clone()), &bytes)?;

  let server_options = DefaultCliOptions {
    mesh: opts.mesh.clone(),
    ..Default::default()
  };

  let config = merge_config(&manifest, &opts.fetch, Some(server_options));

  let code = exec_main(&opts, config).await?;
  if code > 0 {
    return Err(anyhow!(format!("Error code {}", code)));
  }

  Ok(())
}

async fn exec_main(opts: &super::RunCommand, config: WasmflowManifest) -> Result<u32> {
  let host_builder = HostBuilder::from_definition(config);

  let mut host = host_builder.build();
  host.connect_to_mesh().await?;
  host.start_network(opts.seed.map(Seed::unsafe_new)).await?;

  let code = host.exec_main(opts.args.clone()).await?;

  host.stop().await;

  Ok(code)
}
