pub(crate) use vino_interfaces_collection::rm_item::*;
use vino_provider::Context;

pub(crate) async fn job(
  input: Inputs,
  _output: Outputs,
  context: Context<crate::State>,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let mut state = context.lock().unwrap();
  let content_key = format!("{}:{}", input.collection_id, input.document_id);
  state.documents.remove(&content_key);
  Ok(())
}