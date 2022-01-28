use std::fs::read;

use httpmock::MockServer;
use httpmock::prelude::*;
use moka::future::Cache;
use warp::test::request;

use truelayer_coding_challenge::{
  api::API,
  models::poke_models::PokemonResponse,
  util::{TranslationType, MokaCache},
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

  let poke_client = API::new()
    .override_uri(mock.server_address().to_string())
    .disable_https();

  let cache: MokaCache<(String, TranslationType), PokemonResponse> = MokaCache(Cache::new(1_000));
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
      .body_from_file(format!("{}/tests/assets/raw_translation_pikachu.json", ROOT));
  }).await;

  let translation_client = API::new()
    .override_uri(mock.server_address().to_string())
    .disable_https();

  let cache: MokaCache<(String, TranslationType), PokemonResponse> = MokaCache(Cache::new(1_000));
  let router = router(MockPokeAPI, translation_client, cache);

  let res = request().path("/pokemon/translated/pikachu").reply(&router).await;

  assert!(res.status().is_success());

  mock.assert_async().await;
}

#[tokio::test]
async fn test_advanced_handler_yoda() {
  let mock_server = MockServer::start_async().await;

  let mock_diglett = mock_server.mock_async(|when, then| {
    when.method(GET)
      .path("/translate/yoda")
      .query_param("text", "Lives about one yard underground where it feeds on plant roots. It sometimes appears above ground.");
    then.status(200)
      .header("content-type", "application/json")
      .body_from_file(format!("{}/tests/assets/raw_translation_diglett.json", ROOT));
  }).await;

  let mock_regice = mock_server.mock_async(|when, then| {
    when.method(GET)
      .path("/translate/yoda")
      .query_param("text", "REGICE’s body was made during an ice age. The deep-frozen body can’t be melted, even by fire. This POKéMON controls frigid air of minus 328 degrees F.");
    then.status(200)
      .header("content-type", "application/json")
      .body_from_file(format!("{}/tests/assets/raw_translation_regice.json", ROOT));
  }).await;

  let translation_client = API::new()
    .override_uri(mock_diglett.server_address().to_string())
    .disable_https();

  let cache: MokaCache<(String, TranslationType), PokemonResponse> = MokaCache(Cache::new(1_000));
  let router = router(MockPokeAPI, translation_client, cache);

  let res_a = request().path("/pokemon/translated/diglett").reply(&router).await;
  let res_b = request().path("/pokemon/translated/regice").reply(&router).await;

  assert!(res_a.status().is_success());
  assert!(res_b.status().is_success());

  mock_diglett.assert_async().await;
  mock_regice.assert_async().await;
}

#[tokio::test]
async fn test_advanced_handler_rejection() {
  let mock_server = MockServer::start_async().await;

  let mock = mock_server.mock_async(|when, then| {
    when.method(GET)
      .any_request();
    then.status(429);
  }).await;

  let translation_client = API::new()
    .override_uri(mock.server_address().to_string())
    .disable_https();

  let cache: MokaCache<(String, TranslationType), PokemonResponse> = MokaCache(Cache::new(1_000));
  let router = router(MockPokeAPI, translation_client, cache);

  let res = request().path("/pokemon/translated/pikachu").reply(&router).await;

  assert!(res.status().is_success());
  assert_eq!(
    res.body().to_vec(),
    read(format!("{}/tests/assets/expected_pikachu.json", ROOT)).expect("Read test data")
  );

  mock.assert_async().await;
}
