use std::fs::read;

use moka::future::Cache;
use warp::test::request;

use truelayer_coding_challenge::{server::*, util::TranslationType, models::poke_models::PokemonSpecies};

mod mock_impl;
use mock_impl::{MockPokeAPI, MockTranslationAPI};

const ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

#[tokio::test]
async fn basic_handler_test() {
  let cache: Cache<(String, TranslationType), PokemonSpecies> = Cache::new(1_000);
  let router = router(MockPokeAPI, MockTranslationAPI, cache);

  let res = request().path("/pokemon/pikachu").reply(&router).await;

  assert!(res.status().is_success());
  assert_eq!(
    String::from_utf8(res.body().to_vec()).expect("Parse"),
    String::from_utf8(read(format!("{}/tests/assets/expected_pikachu.json", ROOT)).expect("Read test data")).expect("Parse")
  )
}
