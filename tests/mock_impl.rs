use std::fs::read;

use async_trait::async_trait;
use hyper::StatusCode;
use serde_json::from_slice;

use truelayer_coding_challenge::util::{PokeClient, TranslationClient, PokError, TranslationType};
use truelayer_coding_challenge::models::poke_models::PokemonSpecies;

const ROOT: &'static str = env!("CARGO_MANIFEST_DIR");

#[derive(Clone)]
pub(crate) struct MockPokeAPI;

#[async_trait]
impl PokeClient for MockPokeAPI {
  const POKEAPI: & 'static str = "";

  fn get_pokeapi_url(&self) -> String {
    String::from("")
  }

  async fn get_pokemon(&self, pokemon: String) -> Result<PokemonSpecies, PokError> {
    if pokemon == "pikachu" {
      from_slice::<PokemonSpecies>(&read(format!("{}/tests/assets/raw_pikachu.json", ROOT)).expect("Read test data")).map_err(|e| e.into())
    } else {
      Err(PokError::Unavailable(StatusCode::BAD_REQUEST))
    }
  }
}

#[derive(Clone)]
pub(crate) struct MockTranslationAPI;

#[async_trait]
impl TranslationClient for MockTranslationAPI {
  const TRANSLATION_API: & 'static str = "";

  fn get_translation_url(&self) -> String {
    String::from("")
  }

  async fn translate(&self, pokemon: &PokemonSpecies, _translate_to: TranslationType) -> Result<PokemonSpecies, PokError> {
    if pokemon.name() == "pikachu" {
      from_slice::<PokemonSpecies>(&read(format!("{}/tests/assets/translated_pikachu.json", ROOT)).expect("Read test data")).map_err(|e| e.into())
    } else {
      Err(PokError::Unavailable(StatusCode::BAD_REQUEST))
    }
  }

}
