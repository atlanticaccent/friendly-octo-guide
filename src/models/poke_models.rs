use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PokemonSpecies {
  name: String,
  #[serde(alias = "flavor_text_entries")]
  description: Description,
  habitat: NamedAPIResource,
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
  pub fn get_first_description(&self, key: &str) -> Option<&str> {
    match self.description {
      Description::Vec(ref vec) => {
        vec.iter()
          .find(|flavour| flavour.language().name() == key)
          .and_then(|flavour| Some(flavour.flavor_text()))
      },
      Description::String(ref str) => Some(str.as_str())
    }
  }

  // Set the description of this species object to a specific string - for use when sending a
  // modified version back to the user, like when using the translation endpoint.
  pub fn set_description(&mut self, description: String) {
    self.description = Description::String(description)
  }

  /// Get a reference to the pokemon species's habitat.
  pub fn habitat(&self) -> &str {
    &self.habitat.name()
  }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Description {
  Vec(Vec<FlavourText>),
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
