use std::convert::Infallible;

use crate::util::{PokeClient, TranslationClient, TranslationType, handle_reject, PokError, CacheWrapper};
use crate::models::poke_models::PokemonSpecies;

use warp::{Reply, Filter, reject, Rejection, reply::json, path};

pub async fn basic_handler(
  pokemon: String,
  poke_client: impl PokeClient,
  cache: impl CacheWrapper<(String, TranslationType), PokemonSpecies>,
) -> Result<PokemonSpecies, Rejection> {
  if let Some(cached_pokemon) = cache.get(&(pokemon.clone(), TranslationType::None)) {
    return Ok(cached_pokemon)
  }

  let mut species = poke_client
    .get_pokemon(pokemon.clone())
    .await
    .map_err(|err| reject::custom(err))?;

  if let Some(desc) = species.get_first_description("en") {
    species.set_description(desc);
    cache.insert((pokemon, TranslationType::None), species.clone()).await;
    Ok(species)
  } else {
    Err(reject::custom(PokError::NoDescription))
  }
}

pub async fn advanced_handler(
  pokemon: PokemonSpecies,
  translation_client: impl TranslationClient,
  cache: impl CacheWrapper<(String, TranslationType), PokemonSpecies>,
) -> Result<PokemonSpecies, Rejection> {
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
      cache.insert((pokemon.name().to_owned(), translate_to), translated.clone()).await;
      Ok(translated)
    },
    Err(_err) => {
      dbg!(_err);
      Ok(pokemon)
    }
  }
}

fn format(
  mut pokemon: PokemonSpecies
) -> impl Reply {
  pokemon.format_habitat();
  json(&pokemon)
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
  cache: impl CacheWrapper<(String, TranslationType), PokemonSpecies>,
) -> impl Filter<Extract = (impl CacheWrapper<(String, TranslationType), PokemonSpecies>,), Error = Infallible> + Clone {
  warp::any().map(move || cache.clone())
}

pub fn router(
  poke_client: impl PokeClient,
  translation_client: impl TranslationClient,
  cache: impl CacheWrapper<(String, TranslationType), PokemonSpecies>,
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
