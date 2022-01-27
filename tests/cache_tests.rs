use httpmock::{MockServer, Method::GET};
use warp::test::request;

use truelayer_coding_challenge::{
  api::API,
  server::router,
};

mod mock_impl;
use mock_impl::{MockTranslationAPI, MockCache};

const ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

#[tokio::test]
async fn test_cache_utilisation_basic() {
  let mock_server = MockServer::start_async().await;

  let mock = mock_server.mock_async(|when, then| {
    when.method(GET)
      .path("/api/v2/pokemon-species/pikachu");
    then.status(200)
      .header("content-type", "application/json")
      .body_from_file(format!("{}/tests/assets/raw_pikachu.json", ROOT));
  }).await;

  let mock_cache = MockCache::default();
  let poke_client = API::new()
    .override_uri(mock.server_address().to_string())
    .disable_https();

  let router = router(poke_client, MockTranslationAPI, mock_cache.clone());

  let res_a = request().path("/pokemon/pikachu").reply(&router).await;
  let res_b = request().path("/pokemon/pikachu").reply(&router).await;
  let res_c = request().path("/pokemon/pikachu").reply(&router).await;

  assert!(res_a.status().is_success() && res_b.status().is_success() && res_c.status().is_success());

  mock.assert_async().await;

  assert_eq!(*mock_cache.get_count(), 3);
  assert_eq!(*mock_cache.insert_count(), 1);
}
