use std::fs::read;

use serde_json::from_slice;
use truelayer_coding_challenge::models::{poke_models, translation_models};

const ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

#[test]
fn deserialize_poke_api_species() {
  let json = read(format!("{}/tests/assets/raw_pikachu.json", ROOT)).expect("Read test data");

  let species = from_slice::<poke_models::PokemonSpecies>(&json).expect("Parse json");

  assert_eq!(species.name(), "pikachu");
  assert_eq!(species.get_first_description("en"), Some("When several of these POKéMON gather, their electricity could build and cause lightning storms.".to_owned()));
  assert!(!species.is_legendary());
  assert_eq!(species.habitat(), "forest")
}

#[test]
fn deserialize_translation_api_translation() {
  let json = read(format!("{}/tests/assets/translated_pikachu.json", ROOT)).expect("Read test data");

  let translation = from_slice::<translation_models::TranslationUnit>(&json).expect("Parse json");

  assert_eq!(translation.contents().translated(), "At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms.")
}
