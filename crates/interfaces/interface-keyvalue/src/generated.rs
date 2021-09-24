/**********************************************
***** This file is generated, do not edit *****
***********************************************/

pub mod delete {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "delete".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub key: KeyPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("key", "string")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct KeyPortSender {
    port: PortChannel,
  }

  impl Default for KeyPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("key"),
      }
    }
  }
  impl PortSender for KeyPortSender {
    type PayloadType = String;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.key.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod exists {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "exists".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub key: KeyPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("key", "string")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct KeyPortSender {
    port: PortChannel,
  }

  impl Default for KeyPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("key"),
      }
    }
  }
  impl PortSender for KeyPortSender {
    type PayloadType = String;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.key.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod key_get {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "key-get".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub value: ValuePortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("value", "string")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct ValuePortSender {
    port: PortChannel,
  }

  impl Default for ValuePortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("value"),
      }
    }
  }
  impl PortSender for ValuePortSender {
    type PayloadType = String;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.value.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod key_increment {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "key-increment".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
      value: payload.consume("value")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: i32,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map.insert("value".to_owned(), MessageTransport::success(&inputs.value));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string"), ("value", "i32")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub output: OutputPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("output", "i32")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct OutputPortSender {
    port: PortChannel,
  }

  impl Default for OutputPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("output"),
      }
    }
  }
  impl PortSender for OutputPortSender {
    type PayloadType = i32;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.output.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod key_set {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "key-set".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
      value: payload.consume("value")?,
      expires: payload.consume("expires")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "expires")]
    pub expires: u32,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map.insert("value".to_owned(), MessageTransport::success(&inputs.value));
      map.insert(
        "expires".to_owned(),
        MessageTransport::success(&inputs.expires),
      );
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] =
    &[("key", "string"), ("value", "string"), ("expires", "u32")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub key: KeyPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("key", "string")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct KeyPortSender {
    port: PortChannel,
  }

  impl Default for KeyPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("key"),
      }
    }
  }
  impl PortSender for KeyPortSender {
    type PayloadType = String;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.key.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod list_add {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "list-add".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
      value: payload.consume("value")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map.insert("value".to_owned(), MessageTransport::success(&inputs.value));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string"), ("value", "string")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub key: KeyPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("key", "string")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct KeyPortSender {
    port: PortChannel,
  }

  impl Default for KeyPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("key"),
      }
    }
  }
  impl PortSender for KeyPortSender {
    type PayloadType = String;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.key.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod list_range {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "list-range".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
      start: payload.consume("start")?,
      end: payload.consume("end")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "start")]
    pub start: i32,
    #[serde(rename = "end")]
    pub end: i32,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map.insert("start".to_owned(), MessageTransport::success(&inputs.start));
      map.insert("end".to_owned(), MessageTransport::success(&inputs.end));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string"), ("start", "i32"), ("end", "i32")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub values: ValuesPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("values", "[string]")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct ValuesPortSender {
    port: PortChannel,
  }

  impl Default for ValuesPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("values"),
      }
    }
  }
  impl PortSender for ValuesPortSender {
    type PayloadType = Vec<String>;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.values.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod list_remove {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "list-remove".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
      value: payload.consume("value")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map.insert("value".to_owned(), MessageTransport::success(&inputs.value));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string"), ("value", "string")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub value: ValuePortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("value", "string")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct ValuePortSender {
    port: PortChannel,
  }

  impl Default for ValuePortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("value"),
      }
    }
  }
  impl PortSender for ValuePortSender {
    type PayloadType = String;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.value.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod set_add {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "set-add".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
      value: payload.consume("value")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map.insert("value".to_owned(), MessageTransport::success(&inputs.value));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string"), ("value", "string")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub key: KeyPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("key", "string")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct KeyPortSender {
    port: PortChannel,
  }

  impl Default for KeyPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("key"),
      }
    }
  }
  impl PortSender for KeyPortSender {
    type PayloadType = String;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.key.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod set_get {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "set-get".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub values: ValuesPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("values", "[string]")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct ValuesPortSender {
    port: PortChannel,
  }

  impl Default for ValuesPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("values"),
      }
    }
  }
  impl PortSender for ValuesPortSender {
    type PayloadType = Vec<String>;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.values.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod set_intersection {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "set-intersection".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      keys: payload.consume("keys")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("keys".to_owned(), MessageTransport::success(&inputs.keys));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("keys", "[string]")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub keys: KeysPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("keys", "[string]")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct KeysPortSender {
    port: PortChannel,
  }

  impl Default for KeysPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("keys"),
      }
    }
  }
  impl PortSender for KeysPortSender {
    type PayloadType = Vec<String>;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.keys.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod set_remove {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "set-remove".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      key: payload.consume("key")?,
      value: payload.consume("value")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("key".to_owned(), MessageTransport::success(&inputs.key));
      map.insert("value".to_owned(), MessageTransport::success(&inputs.value));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("key", "string"), ("value", "string")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub value: ValuePortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("value", "string")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct ValuePortSender {
    port: PortChannel,
  }

  impl Default for ValuePortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("value"),
      }
    }
  }
  impl PortSender for ValuePortSender {
    type PayloadType = String;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.value.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}
pub mod set_union {
  use serde::{
    Deserialize,
    Serialize,
  };
  pub use vino_provider::native::prelude::*;

  #[must_use]
  pub fn signature() -> ComponentSignature {
    ComponentSignature {
      name: "set-union".to_owned(),
      inputs: PortSignature::from_list(inputs_list()),
      outputs: PortSignature::from_list(outputs_list()),
    }
  }

  pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
    Ok(Inputs {
      keys: payload.consume("keys")?,
    })
  }

  #[derive(Debug, Deserialize, Serialize, Default, Clone)]
  pub struct Inputs {
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
  }

  impl From<Inputs> for TransportMap {
    fn from(inputs: Inputs) -> TransportMap {
      let mut map = TransportMap::new();
      map.insert("keys".to_owned(), MessageTransport::success(&inputs.keys));
      map
    }
  }

  static INPUTS_LIST: &[(&str, &str)] = &[("keys", "[string]")];

  #[must_use]
  pub fn inputs_list() -> &'static [(&'static str, &'static str)] {
    INPUTS_LIST
  }

  #[derive(Debug, Default)]
  pub struct Outputs {
    pub keys: KeysPortSender,
  }

  static OUTPUTS_LIST: &[(&str, &str)] = &[("keys", "[string]")];

  #[must_use]
  pub fn outputs_list() -> &'static [(&'static str, &'static str)] {
    OUTPUTS_LIST
  }

  #[derive(Debug)]
  pub struct KeysPortSender {
    port: PortChannel,
  }

  impl Default for KeysPortSender {
    fn default() -> Self {
      Self {
        port: PortChannel::new("keys"),
      }
    }
  }
  impl PortSender for KeysPortSender {
    type PayloadType = Vec<String>;

    fn get_port(&self) -> Result<&PortChannel, ProviderError> {
      if self.port.is_closed() {
        Err(ProviderError::SendChannelClosed)
      } else {
        Ok(&self.port)
      }
    }

    fn get_port_name(&self) -> String {
      self.port.name.clone()
    }
  }

  #[must_use]
  pub fn get_outputs() -> (Outputs, TransportStream) {
    let mut outputs = Outputs::default();
    let mut ports = vec![&mut outputs.keys.port];
    let stream = PortChannel::merge_all(&mut ports);
    (outputs, stream)
  }
}