use std::fs::read;

use serde_json::from_slice;
use truelayer_coding_challenge::{poke_models, translation_models};

const ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

#[test]
fn deserialize_poke_api_species() {
  let json = read(format!("{}/tests/assets/pikachu.json", ROOT)).expect("Read test data");

  let species = from_slice::<poke_models::PokemonSpecies>(&json).expect("Parse json");

  assert_eq!(species.name(), "pikachu");
  assert_eq!(species.get_first_description("en"), Some("When several of\nthese POKéMON\ngather, their\u{c}electricity could\nbuild and cause\nlightning storms."));
  assert!(!species.is_legendary());
  assert_eq!(species.habitat(), "forest")
}
