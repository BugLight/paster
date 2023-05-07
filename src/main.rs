use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

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

fn config_command(
    mut config: paster::config::PasterConfig,
    key: &str,
    value: String,
) -> Result<()> {
    paster::config::update_config_value(&mut config, &key, value)
        .with_context(|| "Update config failed")?;
    confy::store("paster", None, config).with_context(|| "Store config failed")?;

    Ok(())
}

fn paster_command(
    _config: paster::config::PasterConfig,
    _dest: Option<String>,
    _file: Option<PathBuf>,
) -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let config: paster::config::PasterConfig =
        confy::load("paster", None).with_context(|| "Load config failed")?;

    match args.command {
        Some(Commands::Config { key, value }) => config_command(config, &key, value),
        None => paster_command(config, args.dest, args.file),
    }
}
