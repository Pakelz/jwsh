use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jwsh", version = "0.1", author = "Pakel")]
#[command(about = "CLI Jadwal Sholat")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Tampilkan jadwal sholat berdasarkan nama kota\nContoh penggunaan:\n\
                      jwsh sholat bandung\n\
                      jwsh sholat \"Bandar Lampung\"
                      "
    )]
    Sholat {
        /// Nama kota
        kota: String,
    },
}
