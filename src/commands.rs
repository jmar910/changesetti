use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use anyhow::{Context, Result};

use serde::{Serialize, Deserialize};

use crate::{Language, BumpType, errors::Error};

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {
    pub language: Language
}

#[derive(Serialize, Deserialize)]
struct ChangsetConfig {
  name: String,
  bump: BumpType
}

pub fn init(project_root: &PathBuf, language: &Language) -> Result<()> {
  fs::create_dir(project_root.join(".changesetti"))?;
  let mut config_file = fs::File::create(project_root.join(".changesetti").join("config.json"))?;
  let config = ProjectConfig {
      language: *language
  };
  let config_json = serde_json::to_string_pretty(&config)?;
  config_file.write_all(config_json.as_bytes())?;
  Ok(())
}

pub fn add_changeset(path: &PathBuf, bump_type: &BumpType) -> Result<()> {
  let path = path.join(human_id::id("-", false)).with_extension("md");
  let mut front_matter = String::from("---\n");
  let changeset_config = ChangsetConfig { name: String::from("package-name"), bump: *bump_type };

  let yaml = serde_yaml::to_string(&changeset_config);

  front_matter.push_str(yaml.as_ref().unwrap());
  front_matter.push_str("---\n");

  let mut changeset_file = fs::File::create(path)?;
  changeset_file.write_all(front_matter.as_bytes());
  Ok(())
}
