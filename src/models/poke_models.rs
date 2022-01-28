use regex::{Regex, RegexBuilder};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

use crate::util::PokError;

lazy_static! {
  static ref REMOVE_ESCAPED: Regex = RegexBuilder::new("\u{0a}|\u{0c}").case_insensitive(true).build().unwrap();
}

/// Serde model representing either the response from Pokeapi.
/// 
/// As serde lets us ignore unknown fields, this model is designed to only 
/// extract the fields relevant to any response that would be generated.
/// 
/// The habitat field can be null in some cases, such as Arceus. In these cases 
/// the returned string is just "null".
#[derive(Deserialize)]
pub struct PokemonSpecies {
  name: String,
  #[serde(alias = "flavor_text_entries")]
  descriptions: Vec<FlavourText>,
  /// Undocumented aspect of Pokeapi is that habitat may be null - example, Arceus
  habitat: Option<NamedAPIResource>,
  is_legendary: bool,
}

impl PokemonSpecies {
  /// Get a reference to the pokemon's name.
  pub fn name(&self) -> &str {
    self.name.as_ref()
  }
  
  /// Get the pokemon's legendary status.
  pub fn is_legendary(&self) -> bool {
    self.is_legendary
  }

  /// Get an option that may contain a reference to the first flavor text/description in the language given.
  ///
  /// If no flavor texts were returned from the API, or, more likely, there were
  /// no flavor texts in the given language, then this function returns None.
  pub fn get_first_description(&self, key: &str) -> Option<String> {
    self.descriptions.iter()
      .find(|flavour| flavour.language().name() == key)
      .and_then(|flavour| Some(flavour.flavor_text()))
      .and_then(|flavor| Some(REMOVE_ESCAPED.replace_all(flavor, " ").to_string()))
  }

  /// Get a reference to the pokemon species's habitat.
  /// 
  /// Returns the literal "null" when the response from Pokeapi itself has 
  /// "null" set in the habitat field.
  pub fn habitat(&self) -> &str {
    match &self.habitat {
      Some(resource) => resource.name(),
      None => "null"
    }
  }
}

/// A Name API Resource
/// 
/// Commonly used in Pokeapi to return objects that have a value but may also 
/// refer to another part of the API for a more detailed response.
/// 
/// As with PokemonSpecies, we have no use for the returned URL and so don't 
/// bother including it in the model.
#[derive(Deserialize)]
pub struct NamedAPIResource {
  name: String,
}

impl NamedAPIResource {
  /// Get a reference to the named apiresource's value.
  pub fn name(&self) -> &str {
    self.name.as_ref()
  }
}

/// A flavor text as returned by Pokeapi
/// 
/// Contains a flavor text (which may include a wide range of unicode, including
/// newlines and form feeds) and a Named API Resource representing the language 
/// the flavor text is in.
#[derive(Deserialize)]
pub struct FlavourText {
  flavor_text: String,
  language: NamedAPIResource
}

impl FlavourText {
  /// Get a reference to the actual flavor text.
  pub fn flavor_text(&self) -> &str {
    self.flavor_text.as_ref()
  }
  
  /// Get a reference to the flavour text's language.
  pub fn language(&self) -> &NamedAPIResource {
    &self.language
  }
}

/// The response this API will return following a successful request.
/// 
/// Effectively a stricter version of PokemonSpecies, implementing 
/// TryFrom<PokemonSpecies> to allow conversion between the two, which may or 
/// may not fail.
/// 
/// The use of a separate type ensures that the returned object is always 
/// explicitly a response type, and not just a passed on API response.
#[derive(Serialize, Clone)]
pub struct PokemonResponse {
  name: String,
  description: String,
  habitat: String,
  is_legendary: bool
}

impl PokemonResponse {
  /// Get a reference to the pokemon's name.
  pub fn name(&self) -> &str {
    self.name.as_ref()
  }
  
  /// Get a reference to the pokemon's description.
  pub fn description(&self) -> &str {
    self.description.as_ref()
  }

  /// Get a reference to the pokemon's habitat.
  pub fn habitat(&self) -> &str {
    self.habitat.as_ref()
  }
  
  /// Get whether the pokemon is legendary.
  pub fn is_legendary(&self) -> bool {
    self.is_legendary
  }

  pub fn set_description(&mut self, translated: String) {
    self.description = translated;
  }
}

impl TryFrom<PokemonSpecies> for PokemonResponse {
  type Error = PokError;

  fn try_from(species: PokemonSpecies) -> Result<Self, Self::Error> {
    Ok(Self {
      name: species.name().to_owned(),
      description: species.get_first_description("en").ok_or_else(|| PokError::NoDescription)?,
      habitat: species.habitat().to_owned(),
      is_legendary: species.is_legendary()
    })
  }
}
