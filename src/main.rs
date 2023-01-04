#![allow(unused)]

use std::fs::{File, self};
use std::path::{PathBuf, Path};
use std::env;
use clap::{Parser, Subcommand, ValueEnum};
use plugins::{LanguagePlugin, JavascriptPlugin};
use serde::{Serialize, Deserialize};
use commands::{init, add_changeset, ProjectConfig};

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
        Box::new(JavascriptPlugin {})
      },

      Language::Go => {
        Box::new(JavascriptPlugin {})
      }

      Language::Rust => {
        Box::new(JavascriptPlugin {})
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

enum ProjectError {
  InvalidConfig
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
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let project_root = env::current_dir()?;

    match &cli.command {
        Some(Commands::Init { language }) => {
            init(&project_root, language)
        }

        Some(Commands::Add { bump }) => {
            let changesetti_path = project_root.join(".changesetti");
            add_changeset(&changesetti_path, bump)
        }
        None => {
          Ok(())
        }
    }
}

fn validate_project(changeset_path: &PathBuf) -> Result<(), ProjectError> {
  let config_path = changeset_path.join("config.json");
  let config_file = fs::read(config_path).unwrap();
  let config_str = String::from_utf8(config_file).unwrap();
  let config: ProjectConfig = serde_json::from_str(&config_str).unwrap();
  let language_plugin = config.language.plugin();
  Ok(())
}