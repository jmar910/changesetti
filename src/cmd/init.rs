use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

use crate::Language;
use crate::cmd::ProjectConfig;

pub fn execute(project_root: &PathBuf, language: &Language) -> Result<()> {
  fs::create_dir(project_root.join(".changesetti"))?;
  let mut config_file = fs::File::create(project_root.join(".changesetti").join("config.json"))?;
  let config = ProjectConfig {
      language: *language
  };
  let config_json = serde_json::to_string_pretty(&config)?;
  config_file.write_all(config_json.as_bytes())?;
  Ok(())
}
