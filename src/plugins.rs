
use anyhow::{anyhow, Result};

use std::{io, string, fs, path::Path, process::Command};

use crate::BumpType;

pub trait LanguagePlugin {
  fn validate_language(&self, project_path: &Path) -> Result<()> {
    Ok(())
  }

  fn bump_version(&self, bump: &BumpType) -> Result<String> {
    Ok(String::from(""))
  }

  fn package_name(&self, project_path: &Path) -> Result<String> {
    Ok(String::from(""))
  }
}

pub struct JavascriptPlugin {}
pub struct RubyPlugin {}
pub struct GoPlugin {}
pub struct RustPlugin {}

impl LanguagePlugin for JavascriptPlugin {
  fn validate_language(&self, project_path: &Path) -> Result<()> {
    // TODO: Inspect package.json as an extra layer of validation
    let package_json = fs::read(project_path.join("package.json"))?;
    Ok(())
  }

  fn bump_version(&self, bump: &BumpType) -> Result<String> {
    let bump_str = bump.to_string();
    let output = Command::new("npm").args(["version", &bump_str, "--git-tag-version=false"]).output()?;
    let output_str = String::from_utf8(output.stdout)?;
    let version_str = output_str.trim().strip_prefix('v');

    match version_str {
      Some(version) => { Ok(version.to_string()) }
      None => { Err(anyhow!("Failed to get new bumped version")) }
    }
  }

  fn package_name(&self, project_path: &Path) -> Result<String> {
    let package_path = project_path.join("package.json");
    let node_eval = format!("require('{}').name", package_path.to_string_lossy());
    let output = Command::new("node").args(["-p", &node_eval]).output()?;
    let output_str = String::from_utf8(output.stdout)?;

    Ok(output_str.trim().to_string())
  }
}

impl LanguagePlugin for RubyPlugin {}

impl LanguagePlugin for GoPlugin {}

impl LanguagePlugin for RustPlugin {}
