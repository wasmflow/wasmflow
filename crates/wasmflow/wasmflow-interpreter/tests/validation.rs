use std::path::Path;

mod test;

use anyhow::Result;
use futures::future::BoxFuture;
use seeded_random::Seed;
use serde_json::Value;
use wasmflow_interpreter::graph::from_def;
use wasmflow_interpreter::{
  BoxError,
  Collection,
  HandlerMap,
  Interpreter,
  InterpreterError,
  NamespaceHandler,
  SchematicInvalid,
  ValidationError,
};
use wasmflow_sdk::v1::transport::TransportStream;
use wasmflow_sdk::v1::types::{CollectionFeatures, CollectionSignature, ComponentSignature, TypeSignature};
use wasmflow_sdk::v1::Invocation;
fn load<T: AsRef<Path>>(path: T) -> Result<wasmflow_manifest::WasmflowManifest> {
  Ok(wasmflow_manifest::WasmflowManifest::load_from_file(path.as_ref())?)
}
struct SignatureTestCollection(CollectionSignature);
impl Collection for SignatureTestCollection {
  fn handle(&self, _payload: Invocation, _config: Option<Value>) -> BoxFuture<Result<TransportStream, BoxError>> {
    todo!()
  }

  fn list(&self) -> &CollectionSignature {
    &self.0
  }
}

#[test_logger::test(tokio::test)]
async fn test_missing_collections() -> Result<()> {
  let manifest = load("./tests/manifests/v0/external.wafl")?;
  let network = from_def(&manifest)?;
  let result: std::result::Result<Interpreter, _> = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, None);
  let validation_errors = ValidationError::MissingCollection("test".to_owned());
  if let Err(InterpreterError::EarlyError(e)) = result {
    assert_eq!(e, validation_errors);
  } else {
    panic!()
  }

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_missing_component() -> Result<()> {
  let manifest = load("./tests/manifests/v0/external.wafl")?;
  let network = from_def(&manifest)?;

  let sig = CollectionSignature::default();
  let collections = HandlerMap::new(vec![NamespaceHandler::new(
    "test",
    Box::new(SignatureTestCollection(sig)),
  )]);

  let result: std::result::Result<Interpreter, _> =
    Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections));
  let validation_errors = ValidationError::MissingComponent {
    namespace: "test".to_owned(),
    name: "echo".to_owned(),
  };
  if let Err(InterpreterError::EarlyError(e)) = result {
    assert_eq!(e, validation_errors);
  } else {
    panic!()
  }

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_invalid_port() -> Result<()> {
  let manifest = load("./tests/manifests/v0/external.wafl")?;
  let network = from_def(&manifest)?;
  let signature = CollectionSignature::new("instance")
    .format(1)
    .version("0.0.0")
    .features(CollectionFeatures::v0(false, false))
    .add_component(ComponentSignature::new("echo"));

  let collections = HandlerMap::new(vec![NamespaceHandler::new(
    "test",
    Box::new(SignatureTestCollection(signature)),
  )]);

  let result: std::result::Result<Interpreter, _> =
    Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections));

  if let Err(InterpreterError::EarlyError(e)) = result {
    assert_eq!(
      e,
      ValidationError::MissingConnection {
        port: "input".to_owned(),
        component: "echo".to_owned(),
        namespace: "test".to_owned(),
      }
    );
  } else {
    panic!()
  }

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_missing_port() -> Result<()> {
  let manifest = load("./tests/manifests/v0/external.wafl")?;
  let network = from_def(&manifest)?;
  let signature = CollectionSignature::new("test")
    .format(1)
    .version("0.0.0")
    .features(CollectionFeatures::v0(false, false))
    .add_component(
      ComponentSignature::new("echo")
        .add_input("input", TypeSignature::String)
        .add_input("OTHER_IN", TypeSignature::String)
        .add_output("output", TypeSignature::String)
        .add_output("OTHER_OUT", TypeSignature::String),
    );

  let collections = HandlerMap::new(vec![NamespaceHandler::new(
    "test",
    Box::new(SignatureTestCollection(signature)),
  )]);

  let result: std::result::Result<Interpreter, _> =
    Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections));

  let errors = vec![
    ValidationError::MissingConnection {
      port: "OTHER_IN".to_owned(),
      namespace: "test".to_owned(),
      component: "echo".to_owned(),
    },
    ValidationError::UnusedOutput {
      port: "OTHER_OUT".to_owned(),
      namespace: "test".to_owned(),
      component: "echo".to_owned(),
    },
  ];

  if let Err(InterpreterError::ValidationError(e)) = result {
    assert_eq!(e, vec![SchematicInvalid::new("test".to_owned(), errors)]);
  } else {
    panic!()
  }

  Ok(())
}
