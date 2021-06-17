use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{
  Arc,
  Mutex,
};

use async_trait::async_trait;
use port::Receiver;
use rmp_serde::{
  Deserializer,
  Serializer,
};
use serde::{
  Deserialize,
  Serialize,
};
use vino_guest::OutputPayload;

pub mod error;
pub mod port;
pub mod provider_macro;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = error::ProviderError;
pub type ProviderCallback = Box<dyn Fn(u64, &str, &str, &str, &OutputPayload)>;
pub type Context<T> = Arc<Mutex<T>>;

pub const RPC_OP_INITIALIZE: &str = "rpc_init";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InitConfiguration {
  pub config: HashMap<String, String>,
}

#[async_trait]
pub trait VinoProvider {
  fn init(&self) -> Result<()>;
  async fn request(&self, inv_id: String, component: String, payload: Vec<u8>) -> Result<Receiver>;
}

#[async_trait]
pub trait VinoProviderComponent {
  type Context;
  fn get_name(&self) -> String;
  fn get_input_ports(&self) -> Vec<String>;
  fn get_output_ports(&self) -> Vec<String>;
  async fn job_wrapper(&self, context: Arc<Mutex<Self::Context>>, data: &[u8]) -> Result<Receiver>;
}

/// The agreed-upon standard for RPC response serialization (message pack)
pub fn serialize_rpc_response(item: RPCResponse) -> Result<Vec<u8>> {
  let mut buf = Vec::new();
  match item.serialize(&mut Serializer::new(&mut buf).with_struct_map()) {
    Ok(_) => Ok(buf),
    Err(e) => Err(Error::RPCSerializationError(e)),
  }
}

/// The agreed-upon standard for RPC response de-serialization (message pack)
pub fn deserialize_rpc_response(buf: &[u8]) -> Result<RPCResponse> {
  let mut de = Deserializer::new(Cursor::new(buf));
  match Deserialize::deserialize(&mut de) {
    Ok(t) => Ok(t),
    Err(e) => Err(Error::RPCDeserializationError(e)),
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RPCResponse {
  pub error: Option<String>,
  pub payload: Vec<u8>,
}
