#![deny(
  warnings,
  missing_debug_implementations,
  trivial_casts,
  trivial_numeric_casts,
  unsafe_code,
  unstable_features,
  unused_import_braces,
  unused_qualifications,
  unreachable_pub,
  type_alias_bounds,
  trivial_bounds,
  mutable_transmutes,
  invalid_value,
  explicit_outlives_requirements,
  deprecated,
  clashing_extern_declarations,
  clippy::expect_used,
  clippy::explicit_deref_methods,
  missing_docs
)]
#![warn(clippy::cognitive_complexity)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with_expand_env::with_expand_envs;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A manifest defines the starting state of a Wasmflow host and network.
pub struct WasmflowManifest {
  /// The manifest version.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub version: u8,
  /// Configuration for the host when this manifest is run directly.
  #[serde(default)]
  pub host: HostConfig,
  /// The default flow to execute if none is provided.
  #[serde(default)]
  pub default_flow: Option<String>,
  /// The unique identifier for this manifest.
  #[serde(default)]
  pub name: Option<String>,
  /// The labels and values that apply to this manifest.
  #[serde(default)]
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  #[serde(deserialize_with = "crate::helpers::kv_deserializer")]
  pub labels: HashMap<String, String>,
  /// The collection to use as the entrypoint when running as a standalone process.
  #[serde(default)]
  pub unstable_triggers: Option<EntrypointDefinition>,
  /// A map of namespace to external component collection.
  #[serde(default)]
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  #[serde(deserialize_with = "crate::parse::v1::collection_shortform")]
  pub external: HashMap<String, CollectionDefinition>,
  /// A map of flow-component names to their definitions.
  #[serde(default)]
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  pub components: HashMap<String, FlowDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(tag = "kind")]
/// The possible types of collections.
pub enum CollectionDefinition {
  /// A variant representing a [WasmCollection] type.
  #[serde(rename = "Wasm")]
  WasmCollection(WasmCollection),
  /// A variant representing a [GrpcUrlCollection] type.
  #[serde(rename = "GrpcUrl")]
  GrpcUrlCollection(GrpcUrlCollection),
  /// A variant representing a [GrpcTarCollection] type.
  #[serde(rename = "GrpcTar")]
  GrpcTarCollection(GrpcTarCollection),
  /// A variant representing a [MeshCollection] type.
  #[serde(rename = "Mesh")]
  MeshCollection(MeshCollection),
  /// A variant representing a [ManifestCollection] type.
  #[serde(rename = "Manifest")]
  ManifestCollection(ManifestCollection),
}

#[allow(non_snake_case)]
pub(crate) fn HOST_CONFIG_TIMEOUT() -> u64 {
  5000
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// Host configuration options.
pub struct HostConfig {
  /// Whether or not to allow the :latest tag on remote artifacts.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub allow_latest: bool,
  /// A list of registries to connect to insecurely (over HTTP vs HTTPS).
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub insecure_registries: Vec<String>,
  /// The timeout for network requests (in ms).
  #[serde(default = "HOST_CONFIG_TIMEOUT")]
  #[serde(deserialize_with = "with_expand_envs")]
  pub timeout: u64,
  /// The ID for this host, used to identify the host over the mesh.
  #[serde(default)]
  pub id: Option<String>,
  /// The schematics to expose via RPC or the mesh, if any.
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub expose: Vec<String>,
  /// The mesh configuration.
  #[serde(default)]
  pub mesh: Option<MeshConfig>,
  /// Configuration for the GRPC server.
  #[serde(default)]
  pub rpc: Option<HttpConfig>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// Configuration for the GRPC service.
pub struct HttpConfig {
  /// Enable/disable the server.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub enabled: bool,
  /// The port to bind to.
  #[serde(default)]
  pub port: Option<u16>,
  /// The address to bind to.
  #[serde(default)]
  pub address: Option<String>,
  /// Path to pem file for TLS.
  #[serde(default)]
  pub pem: Option<String>,
  /// Path to key file for TLS.
  #[serde(default)]
  pub key: Option<String>,
  /// Path to CA file.
  #[serde(default)]
  pub ca: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// Configuration used to connect to the mesh.
pub struct MeshConfig {
  /// Enable/disable the mesh connection.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub enabled: bool,
  /// The address of the NATS server.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub address: String,
  /// The path to the NATS credsfile.
  #[serde(default)]
  pub creds_path: Option<String>,
  /// The NATS token.
  #[serde(default)]
  pub token: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A collection definition for the main entrypoint.
pub struct EntrypointDefinition {
  /// The reference/location of the collection.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub reference: String,
  /// The component to use as the entrypoint.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub component: String,
  /// Data or configuration used to initialize the collection.
  #[serde(default)]
  #[serde(deserialize_with = "crate::helpers::deserialize_json_env")]
  pub config: Value,
  /// Permissions to give this collection
  #[serde(default)]
  pub permissions: Permissions,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A WebAssembly collection.
pub struct WasmCollection {
  /// The URL (and optional tag) or local file path to find the .wasm module.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub reference: String,
  /// Permissions to give this collection
  #[serde(default)]
  pub permissions: Permissions,
  /// Per-collection configuration.
  #[serde(default)]
  #[serde(deserialize_with = "crate::helpers::deserialize_json_env")]
  pub config: Value,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// Per-collection permissions configuration.
pub struct Permissions {
  /// A map of from internal directory to external directory that this collection should be able to access.
  #[serde(default)]
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  #[serde(deserialize_with = "crate::helpers::kv_deserializer")]
  pub dirs: HashMap<String, String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A collection hosted as an independent microservice.
pub struct GrpcUrlCollection {
  /// The GRPC URL to connect to.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub url: String,
  /// Any configuration necessary for the collection.
  #[serde(default)]
  #[serde(deserialize_with = "crate::helpers::deserialize_json_env")]
  pub config: Value,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A collection hosted somewhere on a connected mesh.
pub struct MeshCollection {
  /// The ID of the collection.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub id: String,
  /// Any configuration necessary for the collection.
  #[serde(default)]
  #[serde(deserialize_with = "crate::helpers::deserialize_json_env")]
  pub config: Value,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A native collection that can be extracted and run as a microservice.
pub struct GrpcTarCollection {
  /// The URL (and optional tag) or local file path to find the archive.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub reference: String,
  /// Any configuration necessary for the collection.
  #[serde(default)]
  #[serde(deserialize_with = "crate::helpers::deserialize_json_env")]
  pub config: Value,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A native collection that can be extracted and run as a microservice.
pub struct ManifestCollection {
  /// The URL (and optional tag) or local file path to find the manifest.
  #[serde(default)]
  #[serde(deserialize_with = "with_expand_envs")]
  pub reference: String,
  /// Any configuration necessary for the collection.
  #[serde(default)]
  #[serde(deserialize_with = "crate::helpers::deserialize_json_env")]
  pub config: Value,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A definition for an single flow.
pub struct FlowDefinition {
  /// A list of collections the schematic can use.
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub collections: Vec<String>,
  /// A map from component reference to its target.
  #[serde(default)]
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  #[serde(deserialize_with = "crate::parse::v1::map_component_def")]
  pub instances: HashMap<String, ComponentDefinition>,
  /// A list of connections from component to component.
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  #[serde(deserialize_with = "crate::parse::v1::vec_connection")]
  pub flow: Vec<ConnectionDefinition>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A single component definition.
pub struct ComponentDefinition {
  /// The ID of the component (i.e. the alias, key, or namespace).
  #[serde(deserialize_with = "with_expand_envs")]
  pub id: String,
  /// Data to associate with the reference.
  #[serde(default)]
  pub config: Option<Value>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A connection between components. This can be specified in short-form syntax (where applicable). See <a href='https://wasmflow.com/docs/configuration/short-form-syntax/'>wasmflow.com</a> for more information.
pub struct ConnectionDefinition {
  /// The originating component from upstream.
  #[serde(default)]
  #[serde(deserialize_with = "crate::parse::v1::connection_target_shortform")]
  pub from: ConnectionTargetDefinition,
  /// The destination component (downstream).
  #[serde(default)]
  #[serde(deserialize_with = "crate::parse::v1::connection_target_shortform")]
  pub to: ConnectionTargetDefinition,
  /// The default value to provide in the event of an upstream Error or Exception.
  #[serde(default)]
  pub default: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// A connection target e.g. a port on a reference. This can be specified in short-form syntax (where applicable).  See <a href='https://wasmflow.com/docs/configuration/short-form-syntax/'>wasmflow.com</a> for more information.
pub struct ConnectionTargetDefinition {
  /// The instance name of the referenced component.
  #[serde(deserialize_with = "with_expand_envs")]
  pub instance: String,
  /// The component&#x27;s port.
  #[serde(deserialize_with = "with_expand_envs")]
  pub port: String,
  /// The default value to provide on this connection in the event of an error.
  #[serde(default)]
  pub data: Option<Value>,
}
