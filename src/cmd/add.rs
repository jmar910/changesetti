use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

use crate::BumpType;
use crate::cmd::ChangsetConfig;

use super::validate_and_get_config;

pub fn execute(changeset_path: &PathBuf, project_path: &PathBuf, bump_type: &BumpType) -> Result<()> {
  let package_name = validate_and_get_config(changeset_path)?.language.plugin().package_name(project_path)?;
  let path = changeset_path.join(human_id::id("-", false)).with_extension("md");
  let mut front_matter = String::from("---\n");
  let changeset_config = ChangsetConfig { name: package_name, bump: *bump_type };

  let yaml = serde_yaml::to_string(&changeset_config)?;

  front_matter.push_str(&yaml);
  front_matter.push_str("---\n");

  let mut changeset_file = fs::File::create(path)?;
  changeset_file.write_all(front_matter.as_bytes());
  Ok(())
}
