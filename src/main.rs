#![allow(unused)]

use std::fs::{File, self, FileType};
use std::path::{PathBuf, Path};
use std::env;
use std::ffi::OsStr;
use clap::{Parser, Subcommand, ValueEnum};
use errors::Error;
use plugins::{LanguagePlugin, JavascriptPlugin, RubyPlugin, GoPlugin, RustPlugin};
use serde::{Serialize, Deserialize};
use commands::{init, add_changeset, ProjectConfig};
use anyhow::{Context, Result};

pub mod errors;
pub mod commands;
pub mod plugins;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Ruby,
    Javascript,
    Go,
    Rust
}

impl Language {
  fn plugin(&self) -> Box<dyn LanguagePlugin> {
    match *self {
      Language::Javascript => {
        Box::new(JavascriptPlugin {})
      },

      Language::Ruby => {
        Box::new(RubyPlugin {})
      },

      Language::Go => {
        Box::new(GoPlugin {})
      }

      Language::Rust => {
        Box::new(RustPlugin {})
      },
    }

  }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BumpType {
  Patch,
  Minor,
  Major
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        bump: BumpType
    },
    Init {
        #[arg(short, long, value_enum)]
        language: Language
    },
    Version
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let project_root = env::current_dir()?;

    match &cli.command {
        Some(Commands::Init { language }) => {
            init(&project_root, language)
        },

        Some(Commands::Add { bump }) => {
            let changesetti_path = project_root.join(".changesetti");
            validate_project(&changesetti_path, &project_root)?;
            add_changeset(&changesetti_path, bump)
        },

        Some(Commands::Version) => {
            let changeset_path = project_root.join(".changesetti");
            consume_changesets(&changeset_path)
        },

        None => {
          Ok(())
        },
    }
}

fn validate_project(changeset_path: &PathBuf, project_path: &PathBuf) -> Result<()> {
  let config_path = changeset_path.join("config.json");
  let config_file = fs::read(&config_path).with_context(|| format!("Failed to read project config from {}, does it exist?", config_path.display()))?;
  let config_str = String::from_utf8(config_file)?;
  let config: ProjectConfig = serde_json::from_str(&config_str)?;
  let language_plugin = config.language.plugin();
  language_plugin.validate_language(project_path)?;
  Ok(())
}

fn consume_changesets(changeset_path: &PathBuf) -> Result<()> {
  let changeset_dir = fs::read_dir(changeset_path.join(""))?;
  for entry in changeset_dir {
    let entry = entry?;
    let file_type = entry.file_type()?;
    let md_ext_str = OsStr::new("md");
    if file_type.is_file() && entry.path().extension().unwrap().eq(md_ext_str) {
      let changeset_file = fs::read(entry.path());
    }
  }
  Ok(())
}
