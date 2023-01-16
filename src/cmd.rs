use std::path::PathBuf;
use std::fs;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

use crate::{BumpType, Language};

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {
    pub language: Language
}

#[derive(Serialize, Deserialize)]
pub struct ChangsetConfig {
  pub name: String,
  pub bump: BumpType
}

pub fn validate_and_get_config(changeset_path: &PathBuf) -> Result<ProjectConfig> {
  let config_path = changeset_path.join("config.json");
  let config_str = fs::read_to_string(&config_path).with_context(|| format!("Failed to read project config from {}, does it exist?", config_path.display()))?;
  let config: ProjectConfig = serde_json::from_str(&config_str)?;
  Ok(config)
}

pub fn validate_project(changeset_path: &PathBuf, project_path: &PathBuf) -> Result<()> {
  let config: ProjectConfig = validate_and_get_config(changeset_path)?;
  let language_plugin = config.language.plugin();
  language_plugin.validate_language(project_path)?;
  Ok(())
}

pub mod init;
pub mod add;
pub mod version;
