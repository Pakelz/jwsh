use clap::Parser;
use jwsh::show;

use crate::{api::init, cli::Cli};

mod api;
mod cli;
mod helper;
mod models;

fn main() {
    let path = helper::kota_json_path();

    if !path.exists() {
        if let Err(e) = init() {
            eprintln!("Error: {e}");
        }
    }

    let cli = Cli::parse();
    if let Some(kota) = cli.sholat {
        show(kota);
    }
}
