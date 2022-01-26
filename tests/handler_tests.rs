use std::{fs::read, convert::Infallible};

use moka::future::Cache;
use warp::{test::request, Filter, Reply};

use truelayer_coding_challenge::{server::*, util::TranslationType, models::poke_models::PokemonSpecies};

mod mock_impl;
use mock_impl::{MockPokeAPI, MockTranslationAPI};

const ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

fn setup() -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
  let cache: Cache<(String, TranslationType), PokemonSpecies> = Cache::new(1_000);
  router(MockPokeAPI, MockTranslationAPI, cache)
}

#[tokio::test]
async fn basic_handler_test() {
  let router = setup();

  let res = request().path("/pokemon/pikachu").reply(&router).await;

  assert!(res.status().is_success());
  assert_eq!(
    res.body().to_vec(),
    read(format!("{}/tests/assets/expected_pikachu.json", ROOT)).expect("Read test data")
  )
}

#[tokio::test]
async fn advanced_handler_test() {
  let router = setup();

  let res = request().path("/pokemon/translated/pikachu").reply(&router).await;

  assert!(res.status().is_success());
  assert_eq!(
    res.body().to_vec(),
    read(format!("{}/tests/assets/expected_translated_pikachu.json", ROOT)).expect("Read test data")
  )
}
