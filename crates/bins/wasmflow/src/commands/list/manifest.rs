use anyhow::Result;
use wasmflow_collection_cli::options::DefaultCliOptions;
use wasmflow_host::HostBuilder;
use wasmflow_manifest::WasmflowManifest;
use wasmflow_sdk::v1::types::FieldMap;

use crate::utils::merge_config;

pub(crate) async fn handle_command(opts: super::ListCommand, bytes: Vec<u8>) -> Result<()> {
  let manifest = WasmflowManifest::load_from_bytes(Some(opts.location), &bytes)?;

  let server_options = DefaultCliOptions {
    mesh: opts.mesh,
    ..Default::default()
  };

  let mut config = merge_config(&manifest, &opts.fetch, Some(server_options));
  // Disable everything but the mesh
  config.host_mut().rpc = None;

  let host_builder = HostBuilder::from_definition(config);

  let mut host = host_builder.build();
  host.connect_to_mesh().await?;
  host.start_network(None).await?;
  let signature = host.get_signature()?;

  if opts.json {
    let json = serde_json::to_string(&signature)?;
    println!("{}", json);
  } else {
    fn print_component(label: &str, indent: &str, inputs: &FieldMap, outputs: &FieldMap) {
      let inputs = inputs
        .inner()
        .iter()
        .map(|(name, _type)| format!("{}: {:?}", name, _type))
        .collect::<Vec<_>>()
        .join(", ");
      let outputs = outputs
        .inner()
        .iter()
        .map(|(name, _type)| format!("{}: {:?}", name, _type))
        .collect::<Vec<_>>()
        .join(", ");
      println!("{}{}({}) -> ({})", indent, label, inputs, outputs);
    }
    for (_name, component) in signature.components.inner().iter() {
      print!("Component: ");
      print_component(&component.name, "", &component.inputs, &component.outputs);
    }
  }

  Ok(())
}
