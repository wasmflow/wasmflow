pub use crate::components::generated::reverse_uppercase::*;
pub use crate::components::generated::{reverse, uppercase};

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
  async fn job(
    input: Self::Inputs,
    output: Self::Outputs,

    _config: Option<Self::Config>,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reverse_inputs = reverse::Inputs { input: input.input };
    let result = input.link.call("reverse", reverse_inputs).await;
    console_log!("reverse call ok? {}", result.is_ok());
    let mut result: reverse::Outputs = result?.into();

    let payload = result.output().await?.deserialize_next()?;

    let uppercase_inputs = uppercase::Inputs { input: payload };

    let result = input.link.call("uppercase", uppercase_inputs).await;
    console_log!("uppercase call ok? {}", result.is_ok());
    let mut result: uppercase::Outputs = result?.into();

    let payload = result.output().await?.deserialize_next()?;

    output.output.done(payload)?;

    Ok(())
  }
}
