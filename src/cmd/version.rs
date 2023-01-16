use std::ffi::OsStr;
use std::{io::prelude::*, fs::DirEntry};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use yaml_front_matter::{Document, YamlFrontMatter};

use crate::BumpType;
use crate::cmd::{validate_and_get_config, ChangsetConfig};
use crate::sawmill::Sawmill;

pub fn execute(changeset_path: &PathBuf, project_path: &PathBuf) -> Result<()> {
  let change_list = consume_changesets(changeset_path, project_path)?;
  let (changes, dir_entries): (Vec<Document<ChangsetConfig>>, Vec<DirEntry>) = change_list.into_iter().unzip();
  update_changelog(changeset_path, project_path, &changes)?;
  delete_markdown_entries(&dir_entries);
  Ok(())
}

fn consume_changesets(changeset_path: &PathBuf, project_path: &PathBuf) -> Result<Vec<(Document<ChangsetConfig>, DirEntry)>> {
  let changeset_dir = fs::read_dir(changeset_path)?;
  let mut change_list: Vec<(Document<ChangsetConfig>, DirEntry)> = Vec::new();

  for entry in changeset_dir {
    let entry = entry?;
    let file_type = entry.file_type()?;
    let md_ext_str = OsStr::new("md");
    if file_type.is_file() && entry.path().extension().unwrap().eq(md_ext_str) {
      let changset_str = String::from_utf8(fs::read(entry.path())?)?;
      let changeset = YamlFrontMatter::parse::<ChangsetConfig>(&changset_str).unwrap();
      change_list.push((changeset, entry));
    }
  }

  change_list.sort_by(|a, b| b.0.metadata.bump.cmp(&a.0.metadata.bump));

  Ok(change_list)
}

fn update_changelog(changeset_path: &PathBuf, project_path: &PathBuf, changes: &Vec<Document<ChangsetConfig>>) -> Result<()> {
  let language_plugin = validate_and_get_config(changeset_path)?.language.plugin();
  let bumped_version = language_plugin.bump_version(&changes.first().unwrap().metadata.bump)?;

  let major_changes = collect_changes(&changes, &BumpType::Major);
  let minor_changes = collect_changes(&changes, &BumpType::Minor);
  let patch_changes = collect_changes(&changes, &BumpType::Patch);

  let change_groups = vec![&major_changes, &minor_changes, &patch_changes];

  let mut changelog = Sawmill::new(project_path).open_changelog()?;

  changelog.update(&change_groups);
  changelog.close();
  Ok(())
}

fn collect_changes(change_list: &Vec<Document<ChangsetConfig>>, bump_type: &BumpType) -> Vec<String> {
  change_list.iter().filter(|change| change.metadata.bump == *bump_type).map(|change| change.content.to_string()).collect::<Vec<String>>()
}

fn delete_markdown_entries(dir_entries: &Vec<DirEntry>) {
  for dir_entry in dir_entries {
    fs::remove_file(dir_entry.path());
  }
}
