use moka::future::Cache;

extern crate truelayer_coding_challenge;

use truelayer_coding_challenge::util::TranslationType;
use truelayer_coding_challenge::models::poke_models::PokemonSpecies;
use truelayer_coding_challenge::api::{PokeAPI, TranslationAPI};
use truelayer_coding_challenge::server::router;

#[tokio::main]
async fn main() {
  run().await
}

pub async fn run() {
  let cache: Cache<(String, TranslationType), PokemonSpecies> = Cache::new(1_000);
  let poke_client = PokeAPI::new();
  let translation_client = TranslationAPI::new();

  warp::serve(router(poke_client, translation_client, cache))
    .run(([0, 0, 0, 0], 8080))
    .await;
}
