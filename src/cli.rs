use clap::Parser;

#[derive(Parser)]
#[command(name = "jwsh", version = "0.1.1", author = "Pakel")]
#[command(about = "CLI Jadwal Sholat")]
pub struct Cli {
    /// Menampilkan jadwal sholat untuk kota / kabupaten tertentu
    #[arg(short = 's', long = "sholat", value_name = "KOTA")]
    pub sholat: Option<String>,
}
