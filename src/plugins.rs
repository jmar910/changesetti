
use anyhow::Result;

use std::{io, string, fs, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
  #[error("Project does not conform to language in config file")]
  InvalidProjectType
}

pub trait LanguagePlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()>;
  fn bump_version(&self);
}

pub struct JavascriptPlugin {}
pub struct RubyPlugin {}
pub struct GoPlugin {}
pub struct RustPlugin {}

impl LanguagePlugin for JavascriptPlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()> {
    let package_json = fs::read(project_path.join("package.json"))?;
    Ok(())
  }

  fn bump_version(&self) {

  }
}

impl LanguagePlugin for RubyPlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()>{
    Ok(())
  }

  fn bump_version(&self) {

  }
}

impl LanguagePlugin for GoPlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()>{
    Ok(())
  }

  fn bump_version(&self) {

  }
}

impl LanguagePlugin for RustPlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()>{
    Ok(())
  }

  fn bump_version(&self) {

  }
}
