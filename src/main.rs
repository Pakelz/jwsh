use std::path::Path;

use clap::Parser;
use jwsh::{show};

use crate::{api::init, cli::{Cli, Commands}};

mod cli;
mod api;
mod helper;
mod models;

fn main() {
    if !Path::new("kota.json").exists() {
        if let Err(e) = init() {
            eprintln!("Error: {e}");
        }
    }

    let cli = Cli::parse();
    match cli.command {
        Commands::Sholat { kota } => {
            show(kota.to_string());
        }
    }
}
