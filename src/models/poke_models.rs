use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PokemonSpecies {
  name: String,
  #[serde(alias = "flavor_text_entries")]
  description: Description,
  habitat: String,
  is_legendary: bool,
}

impl PokemonSpecies {
  /// Get a reference to the pokemon's name.
  pub fn name(&self) -> &str {
    self.name.as_ref()
  }
  
  /// Get a reference to the pokemon's habitat.
  pub fn habitat(&self) -> &str {
    self.habitat.as_ref()
  }
  
  /// Get the pokemon's legendary status.
  pub fn is_legendary(&self) -> bool {
    self.is_legendary
  }

  pub fn get_first_description(&self) -> Option<&str> {
    match self.description {
      Description::Vec(ref vec) => vec.first().and_then(|flavour| Some(flavour.flavor_text())),
      Description::String(ref str) => Some(str.as_str())
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Description::String(description)
  }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Description {
  Vec(Vec<FlavourText>),
  String(String)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FlavourText {
  flavor_text: String
}

impl FlavourText {
  /// Get a reference to the actual flavor text.
  pub fn flavor_text(&self) -> &str {
      self.flavor_text.as_ref()
  }
}
