use regex::{Regex, RegexBuilder};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

lazy_static! {
  static ref REMOVE_ESCAPED: Regex = RegexBuilder::new("\u{0a}|\u{0c}").case_insensitive(true).build().unwrap();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PokemonSpecies {
  name: String,
  #[serde(alias = "flavor_text_entries")]
  description: Description,
  // Undocumented aspect of Pokeapi is that habitat may be null - example, Arceus
  habitat: Option<Habitat>,
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

  // Get a an option that may contain a reference to the first flavor text/description in the language given.
  // If a description has been set (ie: this is being sent back to the client) then returns the value set.
  pub fn get_first_description(&self, key: &str) -> Option<String> {
    match self.description {
      Description::Vec(ref vec) => {
        vec.iter()
          .find(|flavour| flavour.language().name() == key)
          .and_then(|flavour| Some(flavour.flavor_text()))
          .and_then(|flavor| Some(REMOVE_ESCAPED.replace_all(flavor, " ").to_string()))
      },
      Description::String(ref str) => Some(str.clone())
    }
  }

  // Set the description of this species object to a specific string - for use when sending a
  // modified version back to the user, like when using the translation endpoint.
  pub fn set_description(&mut self, description: String) {
    self.description = Description::String(description)
  }

  // Get a reference to the pokemon species's habitat.
  pub fn habitat(&self) -> &str {
    match &self.habitat {
      Some(Habitat::NamedAPIResource(resource)) => resource.name(),
      Some(Habitat::String(string)) => string,
      None => "null"
    }
  }

  // Set the pokemon's habitat
  pub fn format_habitat(&mut self) {
    if let Some(Habitat::NamedAPIResource(resource)) = &self.habitat {
      self.habitat = Some(Habitat::String(resource.name().to_owned()))
    } else if let None = &self.habitat {
      self.habitat = Some(Habitat::String(String::from("null")))
    }
  }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Description {
  Vec(Vec<FlavourText>),
  String(String)
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Habitat {
  NamedAPIResource(NamedAPIResource),
  String(String)
}

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
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
