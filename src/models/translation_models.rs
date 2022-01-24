use serde::Deserialize;

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
