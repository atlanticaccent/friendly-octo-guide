use std::convert::Infallible;

use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;
use async_trait::async_trait;
use warp::{Reply, Rejection, reject::{Reject, MethodNotAllowed}, reply, body::BodyDeserializeError};

use crate::models::poke_models::PokemonSpecies;

#[async_trait]
pub trait PokeClient: Send + Sync + Clone + 'static {
  const POKEAPI: &'static str;

  fn get_pokeapi_url(&self) -> String;

  async fn get_pokemon(&self, pokemon: String) -> Result<PokemonSpecies, PokError>;
}

#[async_trait]
pub trait TranslationClient: Send + Sync + Clone + 'static {
  const TRANSLATION_API: &'static str;

  fn get_translation_url(&self) -> String;

  async fn translate(&self, pokemon: &PokemonSpecies, translate_to: TranslationType) -> Result<PokemonSpecies, PokError>;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum TranslationType {
  Yoda,
  Shakespeare,
  None
}

impl ToString for TranslationType {
  fn to_string(&self) -> String {
    match self {
      TranslationType::Yoda => String::from("yoda"),
      TranslationType::Shakespeare => String::from("shakespeare"),
      TranslationType::None => unreachable!()
    }
  }
}

#[derive(Error, Debug)]
pub enum PokError {
  #[error("Generic http stream error")]
  Hyper(#[from] hyper::Error),
  #[error("Generic http connection error")]
  Http(#[from] hyper::http::Error),
  #[error("Request failed with status code")]
  Unavailable(hyper::StatusCode),
  #[error("Failed to parse json response")]
  Parse(serde_json::Error),
  #[error("An error ocurred within warp")]
  Warp(#[from] warp::Error),
  #[error("No description for pokemon returned from pokeapi")]
  NoDescription
}

impl From<serde_json::Error> for PokError {
  fn from(err: serde_json::Error) -> Self {
    PokError::Parse(err)
  }
}

#[derive(Serialize)]
struct ErrorReply {
  message: String
}

impl Reject for PokError {}

pub async fn handle_reject(err: Rejection) -> Result<impl Reply, Infallible> {
  let (code, message) = if err.is_not_found() {
    (StatusCode::NOT_FOUND, "Not Found")
  } else if let Some(_) = err.find::<BodyDeserializeError>() {
    (StatusCode::BAD_REQUEST, "Bad Request")
  } else if let Some(_) = err.find::<MethodNotAllowed>() {
    (StatusCode::METHOD_NOT_ALLOWED, "Method not allowed")
  } else if let Some(error) = err.find::<PokError>() {
    match error {
      PokError::Hyper(_) | PokError::Warp(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
      PokError::Parse(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON response from API"),
      PokError::Http(_) | PokError::Unavailable(_) => (StatusCode::BAD_GATEWAY, "Failed to connect to upstream service"),
      PokError::NoDescription => (StatusCode::BAD_GATEWAY, "Pokeapi did not return a description for this pokemon")
    }
  } else {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
  };

  Ok(reply::with_status(
    reply::json(&ErrorReply {
      message: message.to_owned()
    }),
    code
  ))
}
