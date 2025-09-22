use std::path::Path;

use clap::Parser;
use jwsh::show;

use crate::{api::init, cli::Cli};

mod api;
mod cli;
mod helper;
mod models;

fn main() {
    if !Path::new("kota.json").exists() {
        if let Err(e) = init() {
            eprintln!("Error: {e}");
        }
    }

    let cli = Cli::parse();
    if let Some(kota) = cli.sholat {
        show(kota);
    }
}
