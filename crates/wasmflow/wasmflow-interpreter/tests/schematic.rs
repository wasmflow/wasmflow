use std::path::Path;

mod test;

use anyhow::Result;
use seeded_random::Seed;
use test::{JsonWriter, TestCollection};
use tokio_stream::StreamExt;
use wasmflow_interpreter::graph::from_def;
use wasmflow_interpreter::{HandlerMap, Interpreter, InterpreterOptions, NamespaceHandler};
use wasmflow_sdk::v1::packet::PacketMap;
use wasmflow_sdk::v1::transport::MessageTransport;
use wasmflow_sdk::v1::{Entity, Invocation};

fn load<T: AsRef<Path>>(path: T) -> Result<wasmflow_manifest::WasmflowManifest> {
  Ok(wasmflow_manifest::WasmflowManifest::load_from_file(path.as_ref())?)
}

const OPTIONS: Option<InterpreterOptions> = Some(InterpreterOptions {
  error_on_hung: true,
  // TODO: improve logic to ensure no remaining packets are sent after completion.
  // Turn this on to make tests fail in these cases.
  error_on_missing: false,
});

#[test_logger::test(tokio::test)]
async fn test_echo() -> Result<()> {
  let manifest = load("./tests/manifests/v0/echo.wafl")?;
  let network = from_def(&manifest)?;

  let inputs = PacketMap::from([("input", "Hello world".to_owned())]);

  let invocation = Invocation::new_test("echo", Entity::local("echo"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, None)?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.collect().await;
  // let mut outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 2);

  let _wrapper = outputs.pop().unwrap(); //done signal
  let wrapper = outputs.pop().unwrap();
  let result: String = wrapper.deserialize()?;

  assert_eq!(result, "Hello world".to_owned());
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_external_collection() -> Result<()> {
  let manifest = load("./tests/manifests/v0/external.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

  let inputs = PacketMap::from([("input", "Hello world".to_owned())]);

  let invocation = Invocation::new_test("external_collection", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);

  let wrapper = outputs.pop().unwrap();
  let result: String = wrapper.deserialize()?;

  assert_eq!(result, "Hello world".to_owned());
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_self() -> Result<()> {
  let manifest = load("./tests/manifests/v0/reference-self.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

  let inputs = PacketMap::from([("parent_input", "Hello world".to_owned())]);

  let invocation = Invocation::new_test("self", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 1);

  let wrapper = outputs.pop().unwrap();
  let result: String = wrapper.deserialize()?;

  assert_eq!(result, "Hello world".to_owned());
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_exception_default() -> Result<()> {
  let manifest = load("./tests/manifests/v0/exception-default.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
  let inputs = PacketMap::from([("input", "Hello world".to_owned())]);

  let invocation = Invocation::new_test("exception-default", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);

  let wrapper = outputs.pop().unwrap();
  let result: String = wrapper.deserialize()?;

  assert_eq!(result, "eulav tluafeD".to_owned());

  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_exception_nodefault() -> Result<()> {
  let manifest = load("./tests/manifests/v0/exception-nodefault.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
  let inputs = PacketMap::from([("input", "Hello world".to_owned())]);

  let invocation = Invocation::new_test("exception-nodefault", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Failure(_)));

  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_inherent() -> Result<()> {
  let manifest = load("./tests/manifests/v0/inherent.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
  let inputs = PacketMap::default();

  let invocation = Invocation::new_test("inherent", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_inherent_nested() -> Result<()> {
  let manifest = load("./tests/manifests/v0/inherent-nested.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
  let inputs = PacketMap::default();

  let invocation = Invocation::new_test("inherent_nested", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  interpreter.shutdown().await?;
  println!("{:#?}", outputs);

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_inherent_disconnected() -> Result<()> {
  let manifest = load("./tests/manifests/v0/inherent-disconnected.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
  let inputs = PacketMap::from([("input", "Hello world".to_owned())]);

  let invocation = Invocation::new_test("inherent_disconnected", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 1);

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_stream() -> Result<()> {
  let manifest = load("./tests/manifests/v0/stream.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);
  let input_str = "Hello world".to_owned();
  let inputs = PacketMap::from([("input", input_str.clone())]);

  let invocation = Invocation::new_test("stream", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 5);

  for wrapper in outputs {
    let output: String = wrapper.payload.deserialize()?;
    assert_eq!(output, input_str);
  }
  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_spread() -> Result<()> {
  let manifest = load("./tests/manifests/v0/spread.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

  let inputs = PacketMap::from([("input", "Hello world".to_owned())]);
  let invocation = Invocation::new_test("spread", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 2);

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  interpreter.shutdown().await?;

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_generator() -> Result<()> {
  let manifest = load("./tests/manifests/v0/generator.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

  let inputs = PacketMap::default();
  let invocation = Invocation::new_test("generator", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  interpreter.shutdown().await?;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 1);

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_generator_sibling() -> Result<()> {
  let manifest = load("./tests/manifests/v0/generator-sibling.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

  let inputs = PacketMap::from([("input", "my-input".to_owned())]);
  let invocation = Invocation::new_test("generator-sibling", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  interpreter.shutdown().await?;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 1);

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_generator_multi_sibling() -> Result<()> {
  let manifest = load("./tests/manifests/v0/generator-multi-sibling.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

  let inputs = PacketMap::from([
    ("one", "one".to_owned()),
    ("two", "two".to_owned()),
    ("three", "three".to_owned()),
    ("four", "four".to_owned()),
  ]);
  let invocation = Invocation::new_test("generator-sibling", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.drain().await;
  interpreter.shutdown().await?;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 1);

  let wrapper = outputs.pop().unwrap();
  assert!(matches!(wrapper.payload, MessageTransport::Success(_)));

  Ok(())
}

#[test_logger::test(tokio::test)]
#[ignore] // This test won't pass until streaming is re-enabled.
async fn test_stream_collection_ref() -> Result<()> {
  let manifest = load("./tests/manifests/v0/stream-collection-ref.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

  let inputs = PacketMap::from([("input", "my-input".to_owned())]);
  let invocation = Invocation::new_test("stream_collection_ref", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let outputs: Vec<_> = stream.drain().await;
  interpreter.shutdown().await?;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 5);

  for wrapper in outputs {
    assert!(matches!(wrapper.payload, MessageTransport::Success(_)));
  }

  Ok(())
}

#[test_logger::test(tokio::test)]
#[ignore] // This test won't pass until streaming is re-enabled.
async fn test_stream_multi() -> Result<()> {
  let manifest = load("./tests/manifests/v0/stream-multi.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

  let inputs = PacketMap::from([("input", "hello world".to_owned())]);
  let invocation = Invocation::new_test("stream_multi", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let mut stream = interpreter.invoke(invocation).await?;

  let outputs: Vec<_> = stream.drain().await;
  interpreter.shutdown().await?;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 11);

  let (mut vowels, mut rest): (Vec<_>, Vec<_>) = outputs.into_iter().partition(|wrapper| wrapper.port == "vowels");

  let mut expected_vowels: Vec<_> = "eoo".chars().collect();
  while let Some(ch) = expected_vowels.pop() {
    let wrapper = vowels.pop().unwrap();
    assert_eq!(wrapper.payload, MessageTransport::success(&ch));
  }

  let mut expected_other: Vec<_> = "hll wrld".chars().collect();
  while let Some(ch) = expected_other.pop() {
    let wrapper = rest.pop().unwrap();
    assert_eq!(wrapper.payload, MessageTransport::success(&ch));
  }

  Ok(())
}

#[test_logger::test(tokio::test)]
async fn test_no_inputs() -> Result<()> {
  let manifest = load("./tests/manifests/v0/no-inputs.wafl")?;
  let network = from_def(&manifest)?;
  let collections = HandlerMap::new(vec![NamespaceHandler::new("test", Box::new(TestCollection::new()))]);

  let inputs = PacketMap::default();

  let invocation = Invocation::new_test("no-inputs", Entity::local("test"), inputs, None);
  let mut interpreter = Interpreter::new(Some(Seed::unsafe_new(1)), network, None, Some(collections))?;
  interpreter.start(OPTIONS, Some(Box::new(JsonWriter::default()))).await;
  let stream = interpreter.invoke(invocation).await?;

  let mut outputs: Vec<_> = stream.collect().await;
  println!("{:#?}", outputs);
  assert_eq!(outputs.len(), 2);

  let _wrapper = outputs.pop().unwrap(); //done signal
  let wrapper = outputs.pop().unwrap();
  let result: String = wrapper.deserialize()?;

  assert_eq!(result, "Hello world".to_owned());
  interpreter.shutdown().await?;

  Ok(())
}
