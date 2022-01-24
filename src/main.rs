use moka::future::Cache;

mod lib;
mod api;
mod server;

use lib::{TranslationType, poke_models::PokemonSpecies};
use api::{PokeAPI, TranslationAPI};
use server::router;

#[tokio::main]
async fn main() {
  run().await
}

async fn run() {
  let cache: Cache<(String, TranslationType), PokemonSpecies> = Cache::new(1_000);

  let poke_client = PokeAPI::new(cache.clone());
  let translation_client = TranslationAPI::new(cache);

  warp::serve(router(poke_client, translation_client))
    .run(([0, 0, 0, 0], 8080))
    .await;
}
