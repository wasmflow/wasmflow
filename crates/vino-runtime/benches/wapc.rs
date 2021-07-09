use std::collections::HashMap;

use criterion::{
  black_box,
  criterion_main,
  Criterion,
};
use futures::executor::block_on;
use lazy_static::lazy_static;
use maplit::hashmap;
use runtime_utils::{
  init_network_from_yaml,
  TestResult,
};
use vino_entity::Entity;
use vino_runtime::network::Network;

lazy_static! {
  pub static ref NETWORK: Network = {
    let (network, _) = block_on(init_network_from_yaml("./manifests/wapc-component.yaml")).unwrap();
    network
  };
  pub static ref DATA: Data = hashmap! {
      "input".to_owned() => "1234567890".to_owned(),
  };
  pub static ref ENTITY: Entity = Entity::test("wapc_component");
}

pub type Data = HashMap<String, String>;

pub fn bench_async_service(c: &mut Criterion, name: &str) {
  let rt = actix_rt::System::new();

  // start benchmark loops
  c.bench_function(name, move |b| {
    b.iter_custom(|_iters| {
      let start = std::time::Instant::now();
      // benchmark body
      rt.block_on(wapc_component(black_box((
        &NETWORK,
        &DATA,
        ENTITY.to_owned(),
      ))))
      .unwrap();
      // check that at least first request succeeded
      start.elapsed()
    })
  });
}

async fn wapc_component(input: (&Network, &Data, Entity)) -> TestResult<()> {
  let (network, data, entity) = input;
  let _result = network.request("wapc_component", entity, data).await?;
  Ok(())
}

pub fn service_benches() {
  let mut criterion: Criterion<_> = Criterion::default().configure_from_args();
  bench_async_service(&mut criterion, "wapc request");
}

criterion_main!(service_benches);