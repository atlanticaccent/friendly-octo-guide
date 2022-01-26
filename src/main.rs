use moka::future::Cache;

mod util;
mod api;
mod server;

use util::{TranslationType, poke_models::PokemonSpecies};
use api::{PokeAPI, TranslationAPI};
use server::router;

#[tokio::main]
async fn main() {
  run().await
}

async fn run() {
  let cache: Cache<(String, TranslationType), PokemonSpecies> = Cache::new(1_000);
  let poke_client = PokeAPI::new();
  let translation_client = TranslationAPI::new();

  warp::serve(router(poke_client, translation_client, cache))
    .run(([0, 0, 0, 0], 8080))
    .await;
}
