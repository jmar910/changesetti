use crate::errors::Error;

pub trait LanguagePlugin {
  fn validate_language(&self) -> Result<(), Error>;
  fn bump_version(&self);
}

pub struct JavascriptPlugin {}
pub struct RubyPlugin {}
pub struct GoPlugin {}
pub struct RustPlugin {}

impl LanguagePlugin for JavascriptPlugin {
  fn validate_language(&self) -> Result<(), Error> {
    Ok(())
  }

  fn bump_version(&self) {

  }
}

impl LanguagePlugin for RubyPlugin {
  fn validate_language(&self) -> Result<(), Error>{
    Ok(())
  }

  fn bump_version(&self) {

  }
}

impl LanguagePlugin for GoPlugin {
  fn validate_language(&self) -> Result<(), Error>{
    Ok(())
  }

  fn bump_version(&self) {

  }
}

impl LanguagePlugin for RustPlugin {
  fn validate_language(&self) -> Result<(), Error>{
    Ok(())
  }

  fn bump_version(&self) {

  }
}
