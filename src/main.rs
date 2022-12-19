#![allow(unused)]

use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::env;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Language {
    Ruby,
    Javascript,
    Go,
    Rust
}

#[derive(Serialize, Deserialize)]
struct Config {
    language: Language
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
    Add,
    Init {
        #[arg(short, long, value_enum)]
        language: Language
    }
}

trait LanguagePlugin {

}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init { language }) => {
            if let Ok(path) = env::current_dir() {
                init_changesetti(&path, language);
            }
        }

        Some(Commands::Add) => {
            println!("Add command called!!")
        }
        None => {}
    }
}

fn init_changesetti(path: &PathBuf, language: &Language) -> std::io::Result<()> {
    fs::create_dir(path.join(".changesetti"))?;
    let mut config_file = fs::File::create(path.join(".changesetti").join("config.json"))?;
    let config = Config {
        language: *language
    };
    let config_json = serde_json::to_string_pretty(&config)?;
    config_file.write_all(config_json.as_bytes())?;
    Ok(())
}
