use std::convert::Infallible;

use crate::lib::{PokeClient, TranslationClient, TranslationType};
use truelayer_coding_challenge::handle_reject;
use warp::{Reply, Filter, reject, Rejection, reply::json, path};

pub async fn basic_handler(poke_client: impl PokeClient, pokemon: String) -> Result<impl Reply, Rejection> {
  let res = poke_client
    .get_pokemon(pokemon)
    .await
    .map_err(|err| reject::custom(err))?;

  Ok(json(&res))
}

pub async fn advanced_handler(poke_client: impl PokeClient, translation_client: impl TranslationClient, pokemon: String) -> Result<impl Reply, Rejection> {
  let pokemon = poke_client
    .get_pokemon(pokemon)
    .await
    .map_err(|err| reject::custom(err))?;

  let translate_to = if pokemon.is_legendary() || pokemon.habitat() == "cave" {
    TranslationType::Yoda
  } else {
    TranslationType::Shakespeare
  };

  let res = translation_client
    .translate(&pokemon, translate_to)
    .await;

  match res {
    Ok(ref translated) => Ok(json(&translated)),
    Err(_) => Ok(json(&pokemon))
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

pub fn router(
  poke_client: impl PokeClient,
  translation_client: impl TranslationClient
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
  path!("pokemon" / String)
    .and(with_poke_client(poke_client.clone()))
    .map(|pokemon, client| (client, pokemon))
    .untuple_one()
    .and_then(basic_handler)
    .or(
      path!("pokemon" / "translated" / String)
        .and(with_poke_client(poke_client.clone()))
        .and(with_translation_client(translation_client.clone()))
        .map(|pokemon, poke_client, translation_client| {
          (poke_client, translation_client, pokemon)
        })
        .untuple_one()
        .and_then(advanced_handler)
    )
    .recover(handle_reject)
    .with(warp::log("debug"))
}
