use moka::future::Cache;

extern crate truelayer_coding_challenge;

use truelayer_coding_challenge::{
  util::{TranslationType, MokaCache},
  models::poke_models::PokemonResponse,
  api::API,
  server::router,
};

#[tokio::main]
async fn main() {
  run().await
}

pub async fn run() {
  // Cache size set at 1000, as there are just under that many pokemon, with many significantly more popular than others.
  // Additional testing would be required to determine optimal memory/latency settings.
  let cache: MokaCache<(String, TranslationType), PokemonResponse> = MokaCache(Cache::new(1_000));
  let api = API::new();
  let poke_client = api.clone();
  let translation_client = api.clone();

  println!("Starting server on port 8080");
  warp::serve(router(poke_client, translation_client, cache))
    .run(([0, 0, 0, 0], 8080))
    .await;
}
