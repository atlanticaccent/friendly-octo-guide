use std::convert::Infallible;
use core::hash::Hash;

use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;
use async_trait::async_trait;
use warp::{Reply, Rejection, reject::{Reject, MethodNotAllowed}, reply, body::BodyDeserializeError};
use moka::future::Cache;

use crate::models::poke_models::{PokemonSpecies, PokemonResponse};

/// Trait defining the functions an API object needs to contact Pokeapi
/// 
/// Returns either a deserialised API response or an error.
/// 
/// `get_pokemonapi_url` is included as a test helper, allowing test functions 
/// to modify what url an API under test contacts.
#[async_trait]
pub trait PokeClient: Send + Sync + Clone + 'static {
  const POKEAPI: &'static str;

  fn get_pokeapi_url(&self) -> String;

  async fn get_pokemon(&self, pokemon: String) -> Result<PokemonSpecies, PokError>;
}

/// Trait defining the methods an API object needs to contact funtranslations
/// 
/// Returns either a deserialised API response or an error.
/// 
/// `get_translation_url` is included as a test helper, allowing test functions 
/// to modify what url an API under test contacts.
#[async_trait]
pub trait TranslationClient: Send + Sync + Clone + 'static {
  const TRANSLATION_API: &'static str;

  fn get_translation_url(&self) -> String;

  async fn translate(&self, pokemon: &PokemonResponse, translate_to: TranslationType) -> Result<String, PokError>;
}

/// Trait defining cache insertion and get functions
/// 
/// This trait exists solely to improve testing ergonomics - when not testing 
/// this just re-exports the existing cache functions. However, under test, 
/// additional instrumentation can be attached allowing for inspection of cache 
/// utilisation.
#[async_trait]
pub trait CacheWrapper<K, V>: Send + Sync + Clone + 'static
where
  K: Hash + Eq + Send + Sync + 'static,
  V: Clone + Send + Sync + 'static,
{
  fn get(&self, key: &K) -> Option<V>;

  async fn insert(&self, key: K, value: V);
}

/// Non-test implementation of the CacheWrapper trait.
/// 
/// Utilises the moka cache library.
#[derive(Clone)]
pub struct MokaCache<K: Hash + Eq + Send + Sync + 'static, V: Clone + Send + Sync + 'static>(pub Cache<K, V>);

#[async_trait]
impl<K, V> CacheWrapper<K, V> for MokaCache<K, V> 
where
  K: Clone + Hash + Eq + Send + Sync + 'static,
  V: Clone + Send + Sync + 'static,
{
  fn get(&self, key: &K) -> Option<V> {
    self.0.get(key)
  }

  async fn insert(&self, key: K, value: V) {
    self.0.insert(key, value).await
  }
}

/// The type of translation that is being requested
/// 
/// Pokemon that reside in cave habitats and/or are legendary should be 
/// translated under the Yoda scheme.
/// All other pokemon should be translated using the Shakespeare scheme.
/// In the case where an non-translation request is made the type is None.
/// 
/// This is utilised for caching purposes - by keying on not only the pokemon 
/// name but also the translation type, both the untranslated and translated 
/// pokemon objects can be cached simultaneously.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum TranslationType {
  Yoda,
  Shakespeare,
  None
}

/// The to_string function is only used when generating the api path
/// 
/// There should never be a case where a translation request is made with no 
/// translation type set, thus panicing when given None is the most correct 
/// approach as if it ocurrs the application has entered an undefined state.
impl ToString for TranslationType {
  fn to_string(&self) -> String {
    match self {
      TranslationType::Yoda => String::from("yoda"),
      TranslationType::Shakespeare => String::from("shakespeare"),
      TranslationType::None => unreachable!()
    }
  }
}

/// Potential errors that can ocurr during running
/// 
/// The majority are wrappers around existing library error types.
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

/// Handle errors raised at runtime and generate appropriate HTTP error responses
/// 
/// Some of the returned status codes are only approximate, and would ideally 
/// have greater inspection of the actual error.
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
