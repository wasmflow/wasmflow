use std::time::{SystemTime, UNIX_EPOCH};

use runtime_testutils::*;
use tracing::debug;
use wasmflow_sdk::v1::transport::{MessageTransport, TransportWrapper};
use wasmflow_sdk::v1::{Entity, Invocation};
type Result<T> = anyhow::Result<T, anyhow::Error>;
use maplit::hashmap;
use pretty_assertions::assert_eq;

#[test_logger::test(tokio::test)]
async fn good_wapc_component() -> Result<()> {
  let (network, _) = init_network_from_yaml("./manifests/v0/wapc-component.wafl").await?;

  let data = hashmap! {
      "input" => "1234567890",
  };

  println!("Requesting first run");
  let mut result = network
    .invoke(Invocation::new(
      Entity::test("wapc_component"),
      Entity::local("wapc_component"),
      data.try_into()?,
      None,
    ))
    .await?;

  let mut messages: Vec<TransportWrapper> = result.drain_port("output").await?;
  assert_eq!(messages.len(), 1);

  let output: TransportWrapper = messages.pop().unwrap();
  let result: String = output.payload.deserialize()?;
  println!("Output for first run: {:?}", result);
  assert_eq!(result, "1234567890");

  let data = hashmap! {
      "input" => "1234",
  };

  println!("Requesting second run");
  let mut result = network
    .invoke(Invocation::new(
      Entity::test("wapc_component"),
      Entity::local("wapc_component"),
      data.try_into()?,
      None,
    ))
    .await?;

  let mut messages: Vec<TransportWrapper> = result.drain_port("output").await?;
  assert_eq!(messages.len(), 1);

  let output: TransportWrapper = messages.pop().unwrap();

  assert_eq!(
    output.payload,
    MessageTransport::exception("Needs to be longer than 8 characters")
  );

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn good_wasi_component() -> Result<()> {
  let tempdir = std::env::temp_dir();
  let tempfile = tempdir.join("test_file.txt");
  let now = SystemTime::now();
  let time = now.duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
  debug!("Writing '{}' to test file {:?}", time, tempfile);
  std::fs::write(&tempfile, &time).unwrap();
  std::env::set_var("TEST_TEMPDIR", tempdir);
  let (network, _) = init_network_from_yaml("./manifests/v0/wasi-component.wafl").await?;
  std::env::remove_var("TEST_TEMPDIR");

  let data = hashmap! {
      "filename" => "/test_file.txt",
  };

  println!("Requesting first run");
  let mut result = network
    .invoke(Invocation::new(
      Entity::test("wapc_component"),
      Entity::local("wasi_component"),
      data.try_into()?,
      None,
    ))
    .await?;

  let mut messages: Vec<TransportWrapper> = result.drain_port("contents").await?;
  assert_eq!(messages.len(), 1);

  let output: TransportWrapper = messages.pop().unwrap();
  let result: String = output.payload.deserialize()?;
  println!("Output for first run: {:?}", result);
  assert_eq!(result, time);

  Ok(())
}

#[test_logger::test(tokio::test)]
#[ignore]
/// Streams are disabled until updated wapc protocol
async fn wapc_stream() -> Result<()> {
  let (network, _) = init_network_from_yaml("./manifests/v0/wapc-stream.wafl").await?;

  let data = hashmap! {
      "input" => "Hello world",
  };

  println!("Requesting first run");
  let mut result = network
    .invoke(Invocation::new(
      Entity::test("wapc_component"),
      Entity::local("test"),
      data.try_into()?,
      None,
    ))
    .await?;

  let messages: Vec<TransportWrapper> = result.drain_port("output").await?;
  // println!("{:#?}", messages);
  assert_eq!(messages.len(), 5);
  for msg in messages {
    let result: String = msg.payload.deserialize()?;
    assert_eq!(result, "Hello world");
  }

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn bad_wapc_component() -> Result<()> {
  let (network, _) = init_network_from_yaml("./manifests/v0/bad-wapc-component.wafl").await?;

  let data = hashmap! {
      "input" => "1234567890",
  };

  let mut result = network
    .invoke(Invocation::new(
      Entity::test("bad_wapc_component"),
      Entity::local("schematic"),
      data.try_into()?,
      None,
    ))
    .await?;

  let mut messages: Vec<TransportWrapper> = result.drain().await;
  println!("{:#?}", messages);
  assert_eq!(messages.len(), 1);

  let output: TransportWrapper = messages.pop().unwrap();

  println!("output: {:?}", output);
  assert!(output.payload.is_err());
  Ok(())
}

#[test_logger::test(tokio::test)]
async fn wasm_link_call() -> Result<()> {
  let (network, _) = init_network_from_yaml("./manifests/v0/ns-link-wasm.wafl").await?;

  let data = hashmap! {
      "input" => "hello world",
  };

  println!("Requesting first run");
  let mut result = network
    .invoke(Invocation::new(
      Entity::test("ns-link"),
      Entity::local("ns-link"),
      data.try_into()?,
      None,
    ))
    .await?;

  let mut messages = result.drain_port("output").await?;
  println!("{:#?}", messages);
  assert_eq!(messages.len(), 1);

  let output = messages.pop().unwrap();
  let result: String = output.payload.deserialize()?;
  println!("Output for first run: {:?}", result);
  assert_eq!(result, "DLROW OLLEH");

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn subnetwork_link_call() -> Result<()> {
  let (network, _) = init_network_from_yaml("./manifests/v0/subnetwork-ns-link.wafl").await?;

  let data = hashmap! {
      "input" => "hello world",
  };

  let mut result = network
    .invoke(Invocation::new(
      Entity::test("ns-link"),
      Entity::local("test"),
      data.try_into()?,
      None,
    ))
    .await?;

  let mut messages = result.drain_port("output").await?;
  println!("{:#?}", messages);
  assert_eq!(messages.len(), 1);

  let output = messages.pop().unwrap();
  let result: String = output.payload.deserialize()?;
  println!("Output for first run: {:?}", result);
  assert_eq!(result, "DLROW OLLEH");

  Ok(())
}
