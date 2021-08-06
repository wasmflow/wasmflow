use std::panic;

use log::{
  debug,
  error,
};
use serde_json::json;
use utils::*;
use vino_transport::message_transport::{
  JsonError,
  TransportJson,
};

#[test_env_log::test(tokio::test)]
async fn test_vow_serve() -> utils::TestResult<()> {
  debug!("Starting provider");
  let (p_tx, p_handle, port) = start_provider(
    "vow",
    &[
      "serve",
      "./crates/integration/test-wapc-component/build/test_component_s.wasm",
    ],
    &[],
  )
  .await?;
  let input_data = "test input";

  let args = vec![format!("input=\"{}\"", input_data)];
  let actual = vinoc_invoke(&port, "validate", args).await?;

  let expected = hashmap! { "output".to_owned() => TransportJson {
      signal: None,
      error_msg: None,
      error_kind: JsonError::None,
      value: json!(input_data),
    }
  };

  let result = panic::catch_unwind(|| {
    equals!(actual, vec![expected]);
  });

  p_tx.send(Signal::Kill).await?;
  p_handle.await??;
  println!("Provider shut down");

  match result {
    Ok(_) => Ok(()),
    Err(e) => {
      error!("{:?}", e);
      Err(anyhow!("Failed"))
    }
  }
}
