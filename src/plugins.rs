
use anyhow::Result;

use std::{io, string, fs, path::PathBuf, process::Command};

use crate::BumpType;

pub trait LanguagePlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()>;
  fn bump_version(&self, bump: &BumpType) -> Result<String>;
}

pub struct JavascriptPlugin {}
pub struct RubyPlugin {}
pub struct GoPlugin {}
pub struct RustPlugin {}

impl LanguagePlugin for JavascriptPlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()> {
    // TODO: Inspect package.json as an extra layer of validation
    let package_json = fs::read(project_path.join("package.json"))?;
    Ok(())
  }

  fn bump_version(&self, bump: &BumpType) -> Result<String> {
    let bump_str = bump.to_string();
    let output_str = String::from_utf8(Command::new("npm").args(["version", &bump_str]).output()?.stdout)?;

    Ok(output_str.strip_prefix("v").unwrap().to_string())
  }
}

impl LanguagePlugin for RubyPlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()>{
    Ok(())
  }

  fn bump_version(&self, bump: &BumpType) -> Result<String> {
    Ok(String::from(""))
  }
}

impl LanguagePlugin for GoPlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()>{
    Ok(())
  }

  fn bump_version(&self, bump: &BumpType) -> Result<String> {
    Ok(String::from(""))
  }
}

impl LanguagePlugin for RustPlugin {
  fn validate_language(&self, project_path: &PathBuf) -> Result<()>{
    Ok(())
  }

  fn bump_version(&self, bump: &BumpType) -> Result<String> {
    Ok(String::from(""))
  }
}
