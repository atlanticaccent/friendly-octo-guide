use async_trait::async_trait;
use hyper::{Client, client::HttpConnector, Uri, body::to_bytes};
use hyper_tls::HttpsConnector;
use serde_json::from_slice;
use urlencoding::encode;

use super::util::{PokeClient, TranslationClient, TranslationType, PokError};
use super::models::{poke_models::PokemonSpecies, translation_models::TranslationUnit};

#[derive(Clone)]
pub struct PokeAPI {
  client: Client<HttpsConnector<HttpConnector>>
}

impl PokeAPI {
  const POKEAPI: &'static str = "https://pokeapi.co/api/v2/pokemon-species/";

  pub fn new() -> Self {
    Self {
      client: Client::builder()
        .build(HttpsConnector::new())
    }
  }
}

#[async_trait]
impl PokeClient for PokeAPI {
  fn get_pokeapi_url() -> String {
    Self::POKEAPI.to_string()
  }

  async fn get_pokemon(&self, pokemon: String) -> Result<PokemonSpecies, PokError> {
    let res = self.client
      .get(format!("{}{}", Self::POKEAPI, pokemon).parse::<Uri>().expect("Parse URI"))
      .await?;

    if !res.status().is_success() {
      return Err(PokError::Unavailable(res.status()))
    }

    let bytes = to_bytes(res.into_body()).await?;
    let mut species = from_slice::<PokemonSpecies>(&bytes)?;

    if let Some(desc) = species.get_first_description("en") {
      species.set_description(desc)
    } else {
      return Err(PokError::NoDescription)
    }

    Ok(species)
  }
}

#[derive(Clone)]
pub struct TranslationAPI {
  client: Client<HttpsConnector<HttpConnector>>
}

impl TranslationAPI {
  const TRANSLATE: &'static str = "https://api.funtranslations.com/";

  pub fn new() -> Self {
    Self {
      client: Client::builder()
        .build(HttpsConnector::new())
    }
  }
}

#[async_trait]
impl TranslationClient for TranslationAPI {
  fn get_translation_url() -> String {
    Self::TRANSLATE.to_owned()
  }

  async fn translate(&self, pokemon: &PokemonSpecies, translate_to: TranslationType) -> Result<PokemonSpecies, PokError> {
    let desc = pokemon.get_first_description("en").ok_or(PokError::NoDescription)?;

    let res = self.client
      .get(format!("{}{}?text={}", Self::TRANSLATE, translate_to.to_string(), encode(&desc)).parse::<Uri>().expect("Parse URI"))
      .await?;

    let bytes = to_bytes(res.into_body()).await?;
    let translation_unit = from_slice::<TranslationUnit>(&bytes)?;
    let translation = translation_unit.contents().translated().to_owned();

    let mut translated_pokemon = pokemon.clone();
    translated_pokemon.set_description(translation);

    Ok(translated_pokemon)
  }
}
