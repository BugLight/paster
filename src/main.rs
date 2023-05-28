use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use anyhow::{Context, Error, Result};
use clap::{Parser, Subcommand};
use paster::config::PasterConfig;
use paster::paste::Paste;

/// Easily paste from your terminal to services like pastebin.com
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Paste destination name from config
    #[arg(short, long, value_name = "DESTINATION")]
    dest: Option<String>,

    /// Optional file name to paste content from
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Update paster config
    Config { key: String, value: String },
}

fn config_command(mut config: PasterConfig, key: &str, value: String) -> Result<()> {
    paster::config::update_config_value(&mut config, &key, value)
        .with_context(|| "Update config failed")?;
    confy::store("paster", None, config).with_context(|| "Store config failed")?;

    Ok(())
}

fn paster_command(config: PasterConfig, dest: Option<String>, file: Option<PathBuf>) -> Result<()> {
    let dest = dest.unwrap_or(config.default);
    let input: Box<dyn Read> = match file {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(std::io::stdin().lock()),
    };

    let reader = BufReader::new(input);

    match config.dest.get(&dest) {
        None => Err(Error::msg("Unknown destination name")),
        Some(destination) => {
            let paste: Box<dyn Paste> = destination.clone().into();
            let url = paste.paste(Box::new(reader))?;
            println!("{}", url);
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let config: PasterConfig = confy::load("paster", None).with_context(|| "Load config failed")?;

    match args.command {
        Some(Commands::Config { key, value }) => config_command(config, &key, value),
        None => paster_command(config, args.dest, args.file),
    }
}
