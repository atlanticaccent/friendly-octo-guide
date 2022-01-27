use httpmock::MockServer;
use httpmock::prelude::*;
use moka::future::Cache;
use warp::test::request;

use truelayer_coding_challenge::{
  api::API,
  models::poke_models::PokemonSpecies,
  util::TranslationType,
  server::router,
};

mod mock_impl;
use mock_impl::{MockTranslationAPI, MockPokeAPI};

const ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

#[tokio::test]
async fn test_basic_handler_networked() {
  let mock_server = MockServer::start_async().await;

  let mock = mock_server.mock_async(|when, then| {
    when.method(GET)
      .path("/api/v2/pokemon-species/pikachu");
    then.status(200)
      .header("content-type", "application/json")
      .body_from_file(format!("{}/tests/assets/raw_pikachu.json", ROOT));
  }).await;

  let mut poke_client = API::new();
  poke_client.override_uri(mock.server_address().to_string());
  poke_client.disable_https();

  let cache: Cache<(String, TranslationType), PokemonSpecies> = Cache::new(1_000);
  let router = router(poke_client, MockTranslationAPI, cache);

  let res = request().path("/pokemon/pikachu").reply(&router).await;

  assert!(res.status().is_success());

  mock.assert_async().await;
}

#[tokio::test]
async fn test_advanced_handler_networked() {
  let mock_server = MockServer::start_async().await;

  let mock = mock_server.mock_async(|when, then| {
    when.method(GET)
      .path("/translate/shakespeare")
      .query_param("text", "When several of these POKéMON gather, their electricity could build and cause lightning storms.");
    then.status(200)
      .header("content-type", "application/json")
      .body_from_file(format!("{}/tests/assets/raw_translation.json", ROOT));
  }).await;

  let mut translation_client = API::new();
  translation_client.override_uri(mock.server_address().to_string());
  translation_client.disable_https();

  let cache: Cache<(String, TranslationType), PokemonSpecies> = Cache::new(1_000);
  let router = router(MockPokeAPI, translation_client, cache);

  let res = request().path("/pokemon/translated/pikachu").reply(&router).await;

  assert!(res.status().is_success());

  mock.assert_async().await;
}
