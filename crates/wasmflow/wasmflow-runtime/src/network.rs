use std::sync::Arc;
use std::time::Duration;

use seeded_random::{Random, Seed};
use uuid::Uuid;
use wasmflow_manifest::WasmflowManifest;
use wasmflow_mesh::Mesh;
use wasmflow_wascap::KeyPair;

use crate::dev::prelude::*;
use crate::network_service::Initialize;

type Result<T> = std::result::Result<T, RuntimeError>;
#[derive(Debug)]
#[must_use]
pub struct Network {
  pub uid: Uuid,
  inner: Arc<NetworkService>,
  #[allow(unused)]
  kp: KeyPair,
  timeout: Duration,
}

#[derive(Debug)]
#[must_use]
pub struct NetworkInit {
  definition: WasmflowManifest,
  allow_latest: bool,
  allowed_insecure: Vec<String>,
  kp: KeyPair,
  timeout: Duration,
  mesh: Option<Arc<Mesh>>,
  namespace: Option<String>,
  rng_seed: Seed,
}

impl Network {
  pub async fn new_default(definition: WasmflowManifest, seed: &str) -> Result<Self> {
    NetworkBuilder::from_definition(definition, seed)?.build().await
  }

  #[instrument(name = "network", skip_all)]
  pub async fn new(config: NetworkInit) -> Result<Self> {
    trace!(?config, "init");
    let rng = Random::from_seed(config.rng_seed);

    let init = Initialize {
      id: rng.uuid(),
      mesh: config.mesh.clone(),
      manifest: config.definition,
      allowed_insecure: config.allowed_insecure.clone(),
      allow_latest: config.allow_latest,
      timeout: config.timeout,
      namespace: config.namespace,
      rng_seed: rng.seed(),
      event_log: None,
    };
    let service = NetworkService::new(init)
      .await
      .map_err(|e| RuntimeError::InitializationFailed(e.to_string()))?;
    Ok(Self {
      uid: service.id,
      inner: service,
      kp: config.kp,
      timeout: config.timeout,
    })
  }

  pub async fn invoke(&self, invocation: Invocation) -> Result<TransportStream> {
    let time = std::time::SystemTime::now();
    trace!(start_time=%time.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() ,"invocation start");

    let response = tokio::time::timeout(self.timeout, self.inner.invoke(invocation)?)
      .await
      .map_err(|_| NetworkError::Timeout)??;
    trace!(duration_ms=%time.elapsed().unwrap().as_millis(),"invocation complete");

    Ok(response.ok()?)
  }

  pub async fn exec_main(&self, argv: Vec<String>) -> u32 {
    let time = std::time::SystemTime::now();
    trace!("executing main");

    let code = self.inner.exec_main(argv).await;
    trace!(duration_ms=%time.elapsed().unwrap().as_millis(),"main complete");

    code
  }

  pub async fn shutdown(&self) -> Result<()> {
    trace!("network shutting down");
    self.inner.shutdown().await?;

    Ok(())
  }

  pub fn get_signature(&self) -> Result<CollectionSignature> {
    let signature = self.inner.get_signature()?;
    trace!(?signature, "network signature");
    Ok(signature)
  }
}

/// The [NetworkBuilder] builds the configuration for a Wasmflow Network.
#[derive(Debug)]
#[must_use]
pub struct NetworkBuilder {
  allow_latest: bool,
  allowed_insecure: Vec<String>,
  definition: WasmflowManifest,
  kp: KeyPair,
  mesh: Option<Arc<Mesh>>,
  timeout: Duration,
  rng_seed: Option<Seed>,
  namespace: Option<String>,
}

impl NetworkBuilder {
  /// Creates a new network builder from a [NetworkDefinition]
  pub fn from_definition(definition: WasmflowManifest, seed: &str) -> Result<Self> {
    let kp = keypair_from_seed(seed)?;
    Ok(Self {
      allow_latest: definition.allow_latest(),
      allowed_insecure: definition.insecure_registries().clone(),
      definition,
      timeout: Duration::from_secs(5),
      mesh: None,
      namespace: None,
      kp,
      rng_seed: None,
    })
  }

  pub fn timeout(self, timeout: Duration) -> Self {
    Self { timeout, ..self }
  }

  pub fn allow_latest(self, allow_latest: bool) -> Self {
    Self { allow_latest, ..self }
  }

  pub fn allow_insecure(self, allowed_insecure: Vec<String>) -> Self {
    Self {
      allowed_insecure,
      ..self
    }
  }

  pub fn mesh(self, mesh: Arc<Mesh>) -> Self {
    Self {
      mesh: Some(mesh),
      ..self
    }
  }

  pub fn with_seed(self, seed: Seed) -> Self {
    Self {
      rng_seed: Some(seed),
      ..self
    }
  }

  pub fn namespace<T: AsRef<str>>(self, namespace: T) -> Self {
    Self {
      namespace: Some(namespace.as_ref().to_owned()),
      ..self
    }
  }

  /// Constructs an instance of a Wasmflow host.
  pub async fn build(self) -> Result<Network> {
    Network::new(NetworkInit {
      definition: self.definition,
      allow_latest: self.allow_latest,
      allowed_insecure: self.allowed_insecure,
      kp: self.kp,
      timeout: self.timeout,
      namespace: self.namespace,
      mesh: self.mesh,
      rng_seed: self.rng_seed.unwrap_or_else(new_seed),
    })
    .await
  }
}
