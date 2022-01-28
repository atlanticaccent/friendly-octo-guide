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
  // Undocumented aspect of Pokeapi is that habitat may be null - example, Arceus
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
  pub fn get_first_description(&self, key: &str) -> Option<String> {
    self.descriptions.iter()
      .find(|flavour| flavour.language().name() == key)
      .and_then(|flavour| Some(flavour.flavor_text()))
      .and_then(|flavor| Some(REMOVE_ESCAPED.replace_all(flavor, " ").to_string()))
  }

  // Get a reference to the pokemon species's habitat.
  pub fn habitat(&self) -> &str {
    match &self.habitat {
      Some(resource) => resource.name(),
      None => "null"
    }
  }
}

#[derive(Deserialize)]
pub struct NamedAPIResource {
  name: String,
  // url: String
}

impl NamedAPIResource {
  /// Get a reference to the named apiresource's name.
  pub fn name(&self) -> &str {
    self.name.as_ref()
  }
}

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
