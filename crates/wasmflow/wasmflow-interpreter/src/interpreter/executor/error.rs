use uuid::Uuid;

use crate::interpreter::error::StateError;

#[derive(thiserror::Error, Debug)]
pub enum ExecutionError {
  #[error("Error in internal channel: {0}")]
  ChannelError(crate::interpreter::channel::error::Error),
  #[error(transparent)]
  InvalidState(#[from] StateError),
  #[error("Payload does not contain message for port '{0}'")]
  MissingInput(String),
  #[error("Channel send failure")]
  ChannelSend,
  #[error("Transaction '{0}' hung and error_on_hung set")]
  HungTransaction(Uuid),
  #[error("Transaction '{0}' missing")]
  MissingTx(Uuid),
  #[error("{0}")]
  CollectionError(Box<dyn std::error::Error + Send + Sync>),

  #[error("Sender configuration did not include valid data")]
  InvalidSenderData,

  #[error("Configuration for dynamic merge component invalid")]
  InvalidMergeConfig,
}
