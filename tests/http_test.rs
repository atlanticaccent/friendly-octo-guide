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
use mock_impl::MockTranslationAPI;

const ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

#[tokio::test]
async fn test_basic_handler_networked() {
  let mock_server = MockServer::start_async().await;

  let mock = mock_server.mock(|when, then| {
    when.method(GET)
      .path("/api/v2/pokemon-species/pikachu");
    then.status(200)
      .header("content-type", "application/json")
      .body_from_file(format!("{}/tests/assets/raw_pikachu.json", ROOT));
  });

  let mut poke_client = API::new();
  poke_client.override_uri(mock.server_address().to_string());
  poke_client.disable_https();

  let cache: Cache<(String, TranslationType), PokemonSpecies> = Cache::new(1_000);
  let router = router(poke_client, MockTranslationAPI, cache);

  let res = request().path("/pokemon/pikachu").reply(&router).await;

  assert!(res.status().is_success());

  mock.assert();
}
