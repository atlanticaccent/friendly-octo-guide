use std::fs::read;

use serde_json::from_slice;
use truelayer_coding_challenge::models::{poke_models, translation_models};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

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
  let json = read(format!("{}/tests/assets/raw_translation_pikachu.json", ROOT)).expect("Read test data");

  let translation = from_slice::<translation_models::TranslationUnit>(&json).expect("Parse json");

  assert_eq!(translation.contents().translated(), "At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms.")
}

#[test]
fn deserialize_poke_api_species_legendary() {
  let json = read(format!("{}/tests/assets/raw_regice.json", ROOT)).expect("Read test data");

  let species = from_slice::<poke_models::PokemonSpecies>(&json).expect("Parse json");

  assert_eq!(species.name(), "regice");
  assert_eq!(species.get_first_description("en"), Some("REGICE’s body was made during an ice age. The deep-frozen body can’t be melted, even by fire. This POKéMON controls frigid air of minus 328 degrees F.".to_owned()));
  assert!(species.is_legendary());
  assert_eq!(species.habitat(), "cave")
}

#[test]
fn deserialize_poke_api_species_null_habitat() {
  let json = read(format!("{}/tests/assets/raw_arceus.json", ROOT)).expect("Read test data");

  let species = from_slice::<poke_models::PokemonSpecies>(&json).expect("Parse json");

  assert_eq!(species.name(), "arceus");
  assert_eq!(species.get_first_description("en"), Some("It is described in mythology as the Pokémon that shaped the universe with its 1,000 arms.".to_owned()));
  assert!(!species.is_legendary());
  assert_eq!(species.habitat(), "null")
}
