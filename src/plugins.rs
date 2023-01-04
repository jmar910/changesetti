pub trait LanguagePlugin {
  fn validate_language(&self);
  fn bump_version(&self);
}

pub struct JavascriptPlugin {}

impl LanguagePlugin for JavascriptPlugin {
  fn validate_language(&self) {
      
  }

  fn bump_version(&self) {
      
  }
}