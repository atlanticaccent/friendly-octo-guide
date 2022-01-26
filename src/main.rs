use moka::future::Cache;

extern crate truelayer_coding_challenge;

use truelayer_coding_challenge::util::TranslationType;
use truelayer_coding_challenge::models::poke_models::PokemonSpecies;
use truelayer_coding_challenge::api::API;
use truelayer_coding_challenge::server::router;

#[tokio::main]
async fn main() {
  run().await
}

pub async fn run() {
  let cache: Cache<(String, TranslationType), PokemonSpecies> = Cache::new(1_000);
  let api = API::new();
  let poke_client = api.clone();
  let translation_client = api.clone();

  warp::serve(router(poke_client, translation_client, cache))
    .run(([0, 0, 0, 0], 8080))
    .await;
}
