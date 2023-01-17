#![allow(unused)]

use std::fs::{File, self, FileType};
use std::path::{PathBuf, Path};
use std::{env, fmt};
use std::ffi::OsStr;
use clap::{Parser, Subcommand, ValueEnum};
use plugins::{LanguagePlugin, JavascriptPlugin, RubyPlugin, GoPlugin, RustPlugin};
use sawmill::Sawmill;
use serde::{Serialize, Deserialize};
use anyhow::{Context, Result};
use yaml_front_matter::{Document, YamlFrontMatter};

pub mod plugins;
pub mod sawmill;
pub mod cmd;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BumpType {
  Patch,
  Minor,
  Major
}

impl fmt::Display for BumpType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          BumpType::Patch => write!(f, "patch"),
          BumpType::Minor => write!(f, "minor"),
          BumpType::Major => write!(f, "major"),
      }
  }
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
        bump: BumpType,
        summary: String
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
            cmd::init::execute(&project_root, language)
        },

        Some(Commands::Add { bump, summary }) => {
            let changeset_path = project_root.join(".changesetti");
            cmd::validate_project(&changeset_path, &project_root)?;
            cmd::add::execute(&changeset_path, &project_root, bump, &summary)
        },

        Some(Commands::Version) => {
            let changeset_path = project_root.join(".changesetti");
            cmd::version::execute(&changeset_path, &project_root)
        },

        None => {
          Ok(())
        },
    }
}
