use std::path::PathBuf;

use anyhow::Result;
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
    Config {
        key: String,
        value: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Config { key: _, value: _ }) => {
            // Update config value
        },
        None => {
            // Actually paste something
        }
    }

    Ok(())
}
