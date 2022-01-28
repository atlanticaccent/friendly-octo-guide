use serde::Deserialize;

/// The response returned from api.funtranslations
/// 
/// The full response includes an extraneous successes object, which we don't 
/// bother including in the model. The returned translation is still, however, 
/// contained in a nested object.
#[derive(Deserialize)]
pub struct TranslationUnit {
  contents: Contents
}

impl TranslationUnit {
  /// Get a reference to the translation unit's contents.
  pub fn contents(&self) -> &Contents {
    &self.contents
  }
}

/// The actual translated string, as wrapped in a JSON object
#[derive(Deserialize)]
pub struct Contents {
  translated: String
}

impl Contents {
  /// Get a reference to the contents's translated string.
  pub fn translated(&self) -> &str {
    self.translated.as_ref()
  }
}
