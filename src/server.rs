use std::convert::Infallible;

use crate::util::{PokeClient, TranslationClient, TranslationType, handle_reject, CacheWrapper};
use crate::models::poke_models::PokemonResponse;

use warp::{Reply, Filter, reject, Rejection, reply::json, path};

/// Filter for "basic" non-translation API requests
/// 
/// Checks cache for an existing response. If none then attempts to request a 
/// species description for the given pokemon from Pokeapi. If a response is 
/// received successfully from Pokeapi, a response object of our own is created,
/// cached, then returned.
pub async fn basic_handler(
  pokemon: String,
  poke_client: impl PokeClient,
  cache: impl CacheWrapper<(String, TranslationType), PokemonResponse>,
) -> Result<PokemonResponse, Rejection> {
  if let Some(cached_pokemon) = cache.get(&(pokemon.clone(), TranslationType::None)) {
    Ok(cached_pokemon)
  } else {
    let species = poke_client
      .get_pokemon(pokemon.clone())
      .await
      .map_err(|err| reject::custom(err))?;

    let response = PokemonResponse::try_from(species).map_err(|err| reject::custom(err))?;
    cache.insert((pokemon, TranslationType::None), response.clone()).await;

    Ok(response)
  }
}

/// Filter for "advanced", translation API requests
/// 
/// First determines what type of translation should be performed based on the 
/// given pokemon - as this filter always follows the basic filter, it receives 
/// a PokemonResponse and so simply queries this object.
/// Checks cache for an existing translated response. If none, sends a request 
/// to the funtranslations API for a translation of the given Pokemon's 
/// description. If a successful response is received, the given reponse has 
/// it's description replaced with the translation, is cached, then returned.
pub async fn advanced_handler(
  mut pokemon: PokemonResponse,
  translation_client: impl TranslationClient,
  cache: impl CacheWrapper<(String, TranslationType), PokemonResponse>,
) -> Result<PokemonResponse, Rejection> {
  let translate_to = if pokemon.is_legendary() || pokemon.habitat() == "cave" {
    TranslationType::Yoda
  } else {
    TranslationType::Shakespeare
  };

  if let Some(cached_translated) = cache.get(&(pokemon.name().to_owned(), translate_to)) {
    return Ok(cached_translated)
  }

  let res = {
    translation_client
      .translate(&pokemon, translate_to)
      .await
  };

  match res {
    Ok(translated) => {
      pokemon.set_description(translated);

      cache.insert((pokemon.name().to_owned(), translate_to), pokemon.clone()).await;
      Ok(pokemon)
    },
    Err(_err) => {
      Ok(pokemon)
    }
  }
}

/// Filter to format a plain PokemonResponse into a warp Json type, which implements Reply
fn format(
  pokemon: PokemonResponse
) -> impl Reply {
  json(&pokemon)
}

/// Inject PokeClient implementor for handlers to make requests with
fn with_poke_client(
  poke_client: impl PokeClient,
) -> impl Filter<Extract = (impl PokeClient,), Error = Infallible> + Clone {
  warp::any().map(move || poke_client.clone())
}

/// Inject TranslationClient implementor for handlers to make requests with
fn with_translation_client(
  translation_client: impl TranslationClient,
) -> impl Filter<Extract = (impl TranslationClient,), Error = Infallible> + Clone {
  warp::any().map(move || translation_client.clone())
}

/// Inject cache for handlers to insert and retrieve from
fn with_cache(
  cache: impl CacheWrapper<(String, TranslationType), PokemonResponse>,
) -> impl Filter<Extract = (impl CacheWrapper<(String, TranslationType), PokemonResponse>,), Error = Infallible> + Clone {
  warp::any().map(move || cache.clone())
}

/// Full router of available public API endpoints
/// 
/// For each path, the necessary client dependencies are injected, followed by 
/// injection of a cache reference (Moka caches are wrapped in an atomic 
/// reference count).
/// 
/// The "pokemon" route is simple - the PokeClient and cache are injected then 
/// the handler is invoked.
/// The "pokemon/translated" handler effectively is an extension of the 
/// "pokemon" route - the same injection and handling as the "pokemon" route is 
/// performed, then the response handed off to be injected into the advanced 
/// handler along with a TranslationClient implementor and the cache again. In 
/// this way, the advanced handler does not need to duplicate the code to 
/// contact Pokeapi itself, and effectively reuses the basic handler to do so.
pub fn router(
  poke_client: impl PokeClient,
  translation_client: impl TranslationClient,
  cache: impl CacheWrapper<(String, TranslationType), PokemonResponse>,
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
  path!("pokemon" / String)
    .and(with_poke_client(poke_client.clone()))
    .and(with_cache(cache.clone()))
    .and_then(basic_handler)
    .or(
      path!("pokemon" / "translated" / String)
        .and(with_poke_client(poke_client.clone()))
        .and(with_cache(cache.clone()))
        .and_then(basic_handler)
        .and(with_translation_client(translation_client.clone()))
        .and(with_cache(cache.clone()))
        .and_then(advanced_handler)
    )
    .unify()
    .map(|p| format(p))
    .recover(handle_reject)
}
