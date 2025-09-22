mod helper;
use std::io::{self, Write};

use crate::{
    api::get_jadwal,
    ui::{show_jadwal, show_possible_location},
};

mod api;
mod models;
mod ui;

pub fn show(input: String) {
    let test = api::find_city(&input.trim());
    match test {
        Ok(x) if x.0.is_empty() => {
            eprintln!("Harap Masukkan Kota atau Kabupaten yang valid");
        }
        Ok(mut x) if x.0.len() > 2 => {
            show_possible_location(&x);
            print!("Masukkan ID: ");
            io::stdout().flush().unwrap();
            let id2 = loop {
                let mut id2 = String::new();
                io::stdin().read_line(&mut id2).unwrap();
                let id2 = id2.trim().to_string();
                if x.0.iter().any(|item| item == &id2) {
                    break id2;
                } else {
                    print!("Masukkan ID yang tercantum diatas: ");
                    io::stdout().flush().unwrap();
                }
            };
            x.0 = vec![id2];
            print!("\x1B[2J\x1B[1;1H");
            let data = get_jadwal(x.0);
            match data {
                Ok(x) => show_jadwal(x),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        Ok(x) => {
            let data = get_jadwal(x.0);
            match data {
                Ok(x) => show_jadwal(x),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
