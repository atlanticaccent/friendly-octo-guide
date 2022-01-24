use async_trait::async_trait;
use hyper::{Client, client::HttpConnector, Uri, body::to_bytes};
use hyper_tls::HttpsConnector;
use serde_json::from_slice;
use moka::future::Cache;
use urlencoding::encode;

use crate::lib::{poke_models::PokemonSpecies, translation_models::TranslationUnit, PokeClient, TranslationClient, TranslationType, PokError};

#[derive(Clone)]
pub struct PokeAPI {
  cache: Cache<(String, TranslationType), PokemonSpecies>,
  client: Client<HttpsConnector<HttpConnector>>
}

impl PokeAPI {
  const POKEAPI: &'static str = "https://pokeapi.co/api/v2/pokemon-species/";

  pub fn new(cache: Cache<(String, TranslationType), PokemonSpecies>) -> Self {
    Self {
      cache,
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
    if let Some(description) = self.cache.get(&(pokemon.clone(), TranslationType::None)) {
      return Ok(description)
    }

    let res = self.client
      .get(format!("{}{}", Self::POKEAPI, pokemon).parse::<Uri>().expect("Parse URI"))
      .await?;

    if !res.status().is_success() {
      return Err(PokError::Unavailable(res.status()))
    }

    let bytes = to_bytes(res.into_body()).await?;
    let species = from_slice::<PokemonSpecies>(&bytes)?;

    if species.get_first_description().is_none() {
      return Err(PokError::NoDescription)
    }

    self.cache.insert((pokemon, TranslationType::None), species.clone()).await;

    Ok(species)
  }
}

#[derive(Clone)]
pub struct TranslationAPI {
  cache: Cache<(String, TranslationType), PokemonSpecies>,
  client: Client<HttpsConnector<HttpConnector>>
}

impl TranslationAPI {
  const TRANSLATE: &'static str = "https://api.funtranslations.com";

  pub fn new(cache: Cache<(String, TranslationType), PokemonSpecies>) -> Self {
    Self {
      cache,
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
    if let Some(translation) = self.cache.get(&(pokemon.name().to_owned(), translate_to)) {
      return Ok(translation)
    }

    let desc = pokemon.get_first_description().ok_or(PokError::NoDescription)?;

    let res = self.client
      .get(Uri::builder()
        .authority(Self::TRANSLATE)
        .path_and_query(format!("/{}?text={}", translate_to.to_string(), encode(&desc)))
        .build()?
      ).await?;

    let bytes = to_bytes(res.into_body()).await?;
    let translation_unit = from_slice::<TranslationUnit>(&bytes)?;
    let translation = translation_unit.contents().translated().to_owned();

    let mut translated_pokemon = pokemon.clone();
    translated_pokemon.set_description(translation);

    self.cache.insert((translated_pokemon.name().to_owned(), translate_to), translated_pokemon.clone()).await;

    Ok(translated_pokemon)
  }
}
