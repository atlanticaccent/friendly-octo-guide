use async_trait::async_trait;
use hyper::{Client, client::HttpConnector, Uri, body::to_bytes};
use hyper_tls::HttpsConnector;
use serde_json::from_slice;
use urlencoding::encode;

use super::util::{PokeClient, TranslationClient, TranslationType, PokError};
use super::models::{poke_models::PokemonSpecies, translation_models::TranslationUnit};

#[derive(Clone)]
pub struct API {
  client: Client<HttpsConnector<HttpConnector>>,
  uri_override: Option<String>,
  https: bool,
}

impl API {
  pub fn new() -> Self {
    Self {
      client: Client::builder()
        .build(HttpsConnector::new()),
      uri_override: None,
      https: true,
    }
  }

  pub fn override_uri(mut self, over_ride: String) -> Self {
    self.uri_override = Some(over_ride);
    self
  }

  pub fn disable_https(mut self) -> Self {
    self.https = false;
    self
  }
}

#[async_trait]
impl PokeClient for API {
  const POKEAPI: &'static str = "pokeapi.co";

  fn get_pokeapi_url(&self) -> String {
    self.uri_override.clone().unwrap_or(Self::POKEAPI.to_string())
  }

  async fn get_pokemon(&self, pokemon: String) -> Result<PokemonSpecies, PokError> {
    let res = self.client
      .get(Uri::builder()
        .scheme(if self.https { "https" } else { "http" })
        .authority(self.get_pokeapi_url())
        .path_and_query(format!("/api/v2/pokemon-species/{}", pokemon))
        .build()?
      )
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

#[async_trait]
impl TranslationClient for API {
  const TRANSLATION_API: &'static str = "api.funtranslations.com";

  fn get_translation_url(&self) -> String {
    self.uri_override.clone().unwrap_or(Self::TRANSLATION_API.to_string())
  }

  async fn translate(&self, pokemon: &PokemonSpecies, translate_to: TranslationType) -> Result<String, PokError> {
    let desc = pokemon.get_first_description("en").ok_or(PokError::NoDescription)?;

    let res = self.client
      .get(Uri::builder()
        .scheme(if self.https { "https" } else { "http" })
        .authority(self.get_translation_url())
        .path_and_query(format!("/translate/{}?text={}", translate_to.to_string(), encode(&desc)))
        .build()?
      )
      .await?;

    return if !res.status().is_success() {
      Err(PokError::Unavailable(res.status()))
    } else {
      let bytes = to_bytes(res.into_body()).await?;
      let translation_unit = from_slice::<TranslationUnit>(&bytes)?;
      let translation = translation_unit.contents().translated().to_owned();

      Ok(translation)
    }
  }
}
