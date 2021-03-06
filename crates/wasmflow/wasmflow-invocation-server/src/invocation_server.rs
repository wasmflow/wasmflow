use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use std::time::{Duration, Instant};

use parking_lot::RwLock;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::{Response, Status};
use wasmflow_rpc::error::RpcError;
use wasmflow_rpc::rpc::invocation_service_server::InvocationService;
use wasmflow_rpc::rpc::{ListResponse, Output, StatsResponse};
use wasmflow_rpc::{rpc, DurationStatistics, Statistics};

use crate::conversion::make_output;
use crate::SharedRpcHandler;

/// A GRPC server for implementers of [wasmflow_rpc::RpcHandler].
#[derive(Derivative)]
#[derivative(Debug)]
pub struct InvocationServer {
  /// The collection that will handle incoming requests.
  #[derivative(Debug = "ignore")]
  pub collection: SharedRpcHandler,

  stats: RwLock<HashMap<String, Statistics>>,
}

impl InvocationServer {
  /// Constructor.
  #[must_use]
  pub fn new(collection: SharedRpcHandler) -> Self {
    Self {
      collection,
      stats: RwLock::new(HashMap::new()),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum JobResult {
  Success,
  Error,
}

impl InvocationServer {
  fn record_execution<T: AsRef<str>>(&self, job: T, status: JobResult, time: Duration) {
    let mut stats = self.stats.write();
    let job = job.as_ref().to_owned();
    let stat = stats.entry(job.clone()).or_insert_with(|| Statistics {
      name: job,
      runs: 0,
      errors: 0,
      execution_duration: None,
    });
    stat.runs += 1;
    if status == JobResult::Error {
      stat.errors += 1;
    }
    let durations = if stat.execution_duration.is_some() {
      let mut durations = stat.execution_duration.take().unwrap();
      if time < durations.min_time {
        durations.min_time = time;
      } else if time > durations.max_time {
        durations.max_time = time;
      }
      let average = ((durations.average_time * (stat.runs - 1)) + time) / stat.runs;
      durations.average_time = average;
      let total = durations.total_time + time;
      durations.total_time = total;

      durations
    } else {
      DurationStatistics {
        min_time: time,
        max_time: time,
        average_time: time,
        total_time: time,
      }
    };
    stat.execution_duration.replace(durations);
  }
}

#[tonic::async_trait]
impl InvocationService for InvocationServer {
  type InvokeStream = ReceiverStream<Result<Output, Status>>;

  async fn invoke(&self, request: tonic::Request<rpc::Invocation>) -> Result<Response<Self::InvokeStream>, Status> {
    let start = Instant::now();

    let (tx, rx) = mpsc::channel(4);
    let invocation = request.into_inner();
    debug!(
      "RPC:Invocation for target {}, message: {:?}",
      invocation.target, invocation.payload
    );
    let invocation_id = invocation.id.clone();
    let entity = wasmflow_sdk::v1::Entity::from_str(&invocation.target);
    if let Err(e) = entity {
      tx.send(Err(Status::failed_precondition(e.to_string()))).await.unwrap();
    } else {
      let invocation: wasmflow_sdk::v1::Invocation = invocation.try_into().map_err(|e| {
        Status::failed_precondition(format!(
          "Could not convert invocation payload into internal data structure: {}",
          e
        ))
      })?;
      let entity = entity.unwrap();
      let entity_name = entity.name().to_owned();

      let result = self.collection.invoke(invocation).await;
      if let Err(e) = result {
        let message = e.to_string();
        error!("Invocation failed: {}", message);
        tx.send(Err(Status::internal(message))).await.unwrap();
        self.record_execution(entity_name, JobResult::Error, start.elapsed());
      } else {
        tokio::spawn(async move {
          let mut receiver = result.unwrap();
          while let Some(next) = receiver.next().await {
            let port_name = next.port;
            let msg = next.payload;
            tx.send(make_output(&port_name, &invocation_id, msg)).await.unwrap();
          }
        });
        self.record_execution(entity_name, JobResult::Success, start.elapsed());
      }
    }

    Ok(Response::new(ReceiverStream::new(rx)))
  }

  async fn list(&self, _request: tonic::Request<rpc::ListRequest>) -> Result<Response<ListResponse>, Status> {
    trace!("Listing registered components from collection");
    let list = self
      .collection
      .get_list()
      .map_err(|e| Status::internal(e.to_string()))?;
    trace!("Server: list is {:?}", list);

    let result: Result<Vec<_>, RpcError> = list.into_iter().map(TryFrom::try_from).collect();
    let schemas = result.map_err(|e| Status::internal(e.to_string()))?;
    let response = ListResponse { schemas };
    Ok(Response::new(response))
  }

  async fn stats(&self, _request: tonic::Request<rpc::StatsRequest>) -> Result<Response<StatsResponse>, Status> {
    Ok(Response::new(StatsResponse {
      stats: self.stats.read().values().cloned().map(From::from).collect(),
    }))
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use anyhow::Result;
  use test_native_collection::Collection;
  use tokio_stream::wrappers::ReceiverStream;
  use tonic::Status;
  use wasmflow_rpc::rpc::{Output, StatsResponse};
  use wasmflow_sdk::v1::packet::PacketMap;
  use wasmflow_sdk::v1::{Entity, Invocation};

  use super::{InvocationServer, InvocationService};

  fn get_server() -> InvocationServer {
    let collection = Arc::new(Collection::default());
    InvocationServer::new(collection)
  }

  async fn make_test_invocation(
    server: &InvocationServer,
  ) -> Result<tonic::Response<ReceiverStream<Result<Output, Status>>>> {
    let payload = PacketMap::from([("input", "hello")]);

    let invocation = Invocation::new_test("stats", Entity::local("test-component"), payload, None);
    let request = tonic::Request::new(invocation.try_into()?);

    let result = server.invoke(request).await?;
    Ok(result)
  }

  async fn get_test_stats() -> Result<StatsResponse> {
    let server = get_server();
    let _response = make_test_invocation(&server).await?;
    let _response = make_test_invocation(&server).await?;
    let _response = make_test_invocation(&server).await?;

    let stats_request = tonic::Request::new(wasmflow_rpc::rpc::StatsRequest {});

    let stats = server.stats(stats_request).await?;
    Ok(stats.into_inner())
  }

  #[test_logger::test(tokio::test)]
  async fn test_stats() -> Result<()> {
    let mut stats = get_test_stats().await?;

    let stat = stats.stats[0].execution_statistics.take().unwrap();

    //three runs must be longer than two runs
    assert!(stat.total > stat.min + stat.max);

    Ok(())
  }
}
