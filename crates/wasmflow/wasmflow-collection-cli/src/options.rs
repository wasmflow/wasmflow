use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::time::Duration;

use clap::Args;
use logger::LoggingOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
/// Server configuration options.
pub struct Options {
  /// RPC server options.
  pub rpc: Option<ServerOptions>,
  /// Mesh options.
  pub mesh: Option<MeshOptions>,
  /// The ID of the server.
  pub id: String,
  /// The timeout for network requests.
  pub timeout: Duration,
}

impl Default for Options {
  fn default() -> Self {
    Self {
      id: uuid::Uuid::new_v4().as_hyphenated().to_string(),
      rpc: Default::default(),
      mesh: Default::default(),
      timeout: Default::default(),
    }
  }
}

#[derive(Debug, Default, Clone)]
/// Configuration used to connect to the mesh
pub struct MeshOptions {
  /// Enable/disable the mesh connection.
  pub enabled: bool,

  /// The address of the NATS server.
  pub address: String,

  /// The path to the NATS credsfile.
  pub creds_path: Option<PathBuf>,

  /// The NATS token.
  pub token: Option<String>,
}

#[derive(Debug, Clone, Default)]
/// Options to use when starting an RPC or HTTP server.
pub struct ServerOptions {
  /// Enable/disable the server.
  pub enabled: bool,

  /// The port to bind to.
  pub port: Option<u16>,

  /// The address to bind to.
  pub address: Option<Ipv4Addr>,

  /// Path to pem file for TLS.
  pub pem: Option<PathBuf>,

  /// Path to key file for TLS.
  pub key: Option<PathBuf>,

  /// Path to CA file.
  pub ca: Option<PathBuf>,
}

impl From<DefaultCliOptions> for Options {
  fn from(opts: DefaultCliOptions) -> Self {
    let rpc = Some(ServerOptions {
      enabled: opts.rpc_enabled,
      port: opts.rpc_port,
      address: opts.rpc_address,
      pem: opts.rpc_pem,
      key: opts.rpc_key,
      ca: opts.rpc_ca,
    });

    #[allow(clippy::option_if_let_else)]
    let mesh = if let Some(url) = opts.mesh.nats_url {
      Some(MeshOptions {
        enabled: opts.mesh.mesh_enabled,
        address: url,
        creds_path: opts.mesh.nats_credsfile,
        token: opts.mesh.nats_token,
      })
    } else {
      None
    };

    let id = opts
      .id
      .unwrap_or_else(|| uuid::Uuid::new_v4().as_hyphenated().to_string());

    Options {
      rpc,
      timeout: Duration::from_millis(opts.timeout.unwrap_or(5000)),
      id,
      mesh,
    }
  }
}

impl From<DefaultCliOptions> for LoggingOptions {
  fn from(opts: DefaultCliOptions) -> Self {
    opts.logging
  }
}

/// Names of the environment variables used for fallback values.
pub mod env {
  macro_rules! env_var {
    (  $x:ident  ) => {
      /// Environment variable fallback for CLI options
      pub const $x: &str = stringify!($x);
    };
  }

  env_var!(WAFL_COLLECTION_ID);
  env_var!(WAFL_TIMEOUT);

  env_var!(WAFL_RPC_ENABLED);
  env_var!(WAFL_RPC_PORT);
  env_var!(WAFL_RPC_ADDRESS);
  env_var!(WAFL_RPC_KEY);
  env_var!(WAFL_RPC_PEM);
  env_var!(WAFL_RPC_CA);

  // Unused for now.
  // env_var!(WAFL_HTTP_ENABLED);
  // env_var!(WAFL_HTTP_PORT);
  // env_var!(WAFL_HTTP_ADDRESS);
  // env_var!(WAFL_HTTP_KEY);
  // env_var!(WAFL_HTTP_PEM);
  // env_var!(WAFL_HTTP_CA);

  env_var!(NATS_URL);
  env_var!(NATS_CREDSFILE);
  env_var!(NATS_TOKEN);
}

#[derive(Debug, Clone, Default, Args, Serialize, Deserialize)]
/// Command line options for s.
pub struct DefaultCliOptions {
  /// The unique ID of this client.
  #[clap(long = "id", env = env::WAFL_COLLECTION_ID, action)]
  pub id: Option<String>,

  /// The timeout for outbound requests in ms.
  #[clap(long = "timeout", env = env::WAFL_TIMEOUT, action)]
  pub timeout: Option<u64>,

  /// Logging options.
  #[clap(flatten)]
  pub logging: LoggingOptions,

  #[clap(flatten)]
  /// Options for connecting to a mesh.
  pub mesh: MeshCliOptions,

  /// Enable the rpc server.
  #[clap(long = "rpc",  env = env::WAFL_RPC_ENABLED, action)]
  pub rpc_enabled: bool,

  /// Port to listen on for GRPC server.
  #[clap(long = "rpc-port", env = env::WAFL_RPC_PORT, action)]
  pub rpc_port: Option<u16>,

  /// IP address to bind to for GRPC server.
  #[clap(long = "rpc-address", env = env::WAFL_RPC_ADDRESS, action)]
  pub rpc_address: Option<Ipv4Addr>,

  /// Path to pem file for TLS for GRPC server.
  #[clap(long = "rpc-pem", env = env::WAFL_RPC_PEM, action)]
  pub rpc_pem: Option<PathBuf>,

  /// Path to key file for TLS for GRPC server.
  #[clap(long = "rpc-key", env = env::WAFL_RPC_KEY, action)]
  pub rpc_key: Option<PathBuf>,

  /// Path to certificate authority for GRPC server.
  #[clap(long = "rpc-ca", env = env::WAFL_RPC_CA, action)]
  pub rpc_ca: Option<PathBuf>,
}

#[derive(Debug, Clone, Default, Args, Serialize, Deserialize)]
/// Command line options for mesh connections.
pub struct MeshCliOptions {
  /// Enable the mesh connection.
  #[clap(long = "mesh", action)]
  pub mesh_enabled: bool,

  /// The url of the NATS server (in IP:PORT format).
  #[clap(long = "nats", env = env::NATS_URL, action)]
  pub nats_url: Option<String>,

  /// The path to the NATS credsfile.
  #[clap(long = "nats-credsfile", env = env::NATS_CREDSFILE, action)]
  pub nats_credsfile: Option<PathBuf>,

  /// The NATS token.
  #[clap(long = "nats-token", env = env::NATS_TOKEN, hide_env_values = true, action)]
  pub nats_token: Option<String>,
}
