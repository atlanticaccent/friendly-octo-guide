use std::convert::Infallible;

use crate::util::{PokeClient, TranslationClient, TranslationType, poke_models::PokemonSpecies, handle_reject, PokError};
use moka::future::Cache;
use warp::{Reply, Filter, reject, Rejection, reply::json, path};

pub async fn basic_handler(
  pokemon: String,
  poke_client: impl PokeClient,
  cache: Cache<(String, TranslationType), PokemonSpecies>,
) -> Result<PokemonSpecies, Rejection> {
  let res = if let Some(cached_pokemon) = cache.get(&(pokemon.clone(), TranslationType::None)) {
    cached_pokemon
  } else {
    poke_client
      .get_pokemon(pokemon)
      .await
      .map_or_else(
        |err| Err(reject::custom(err)),
        |mut species| {
          if let Some(desc) = species.get_first_description("en") {
            species.set_description(desc);
            Ok(species)
          } else {
            Err(reject::custom(PokError::NoDescription))
          }
        }
      )?
  };

  Ok(res)
}

pub async fn advanced_handler(
  pokemon: PokemonSpecies,
  translation_client: impl TranslationClient,
  cache: Cache<(String, TranslationType), PokemonSpecies>,
) -> Result<PokemonSpecies, Rejection> {
  let translate_to = if pokemon.is_legendary() || pokemon.habitat() == "cave" {
    TranslationType::Yoda
  } else {
    TranslationType::Shakespeare
  };

  let res = if let Some(cached_translated) = cache.get(&(pokemon.name().to_owned(), translate_to)) {
    Ok(cached_translated)
  } else {
    translation_client
      .translate(&pokemon, translate_to)
      .await
  };


  match res {
    Ok(translated) => Ok(translated),
    Err(_err) => {
      dbg!(_err);
      Ok(pokemon)
    }
  }
}

fn with_poke_client(
  poke_client: impl PokeClient,
) -> impl Filter<Extract = (impl PokeClient,), Error = Infallible> + Clone {
  warp::any().map(move || poke_client.clone())
}

fn with_translation_client(
  translation_client: impl TranslationClient,
) -> impl Filter<Extract = (impl TranslationClient,), Error = Infallible> + Clone {
  warp::any().map(move || translation_client.clone())
}

fn with_cache(
  cache: Cache<(String, TranslationType), PokemonSpecies>,
) -> impl Filter<Extract = (Cache<(String, TranslationType), PokemonSpecies>,), Error = Infallible> + Clone {
  warp::any().map(move || cache.clone())
}

pub fn router(
  poke_client: impl PokeClient,
  translation_client: impl TranslationClient,
  cache: Cache<(String, TranslationType), PokemonSpecies>,
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
    .map(|res| json(&res))
    .recover(handle_reject)
}
