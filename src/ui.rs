use prettytable::{row, Table};

use crate::models::Jadwal;

pub fn show_possible_location(tuple: &(Vec<String>, Vec<String>)) {
    let mut table = Table::new();
    table.add_row(row![bFy -> "ID", bFy -> "Nama Kota/Kabupaten"]);
    for i in 0..tuple.0.len() {
        table.add_row(row![tuple.0[i], tuple.1[i]]);
    }
    table.printstd();
}

pub fn show_jadwal(jadwal: Vec<Jadwal>) {
    let mut table = Table::new();
    if jadwal.len() < 2 {
        for data in jadwal.iter() {
            table.add_row(row![bFy -> data.data.lokasi]);
            table.add_row(row!["Subuh", data.data.jadwal.subuh]);
            table.add_row(row!["Dzuhur", data.data.jadwal.dzuhur]);
            table.add_row(row!["Ashar", data.data.jadwal.ashar]);
            table.add_row(row!["Maghrib", data.data.jadwal.maghrib]);
            table.add_row(row!["Isya", data.data.jadwal.isya]);
        }
        table.printstd();
    } else if jadwal.len() == 2 {
        let (kota, kabupaten) = (&jadwal[0], &jadwal[1]);
        table.add_row(row![bFb -> "Sholat", bFb -> kota.data.lokasi, bFb -> kabupaten.data.lokasi]);
        table.add_row(row![
            "Subuh",
            kota.data.jadwal.subuh,
            kabupaten.data.jadwal.subuh
        ]);
        table.add_row(row![
            "Dzuhur",
            kota.data.jadwal.dzuhur,
            kabupaten.data.jadwal.dzuhur
        ]);
        table.add_row(row![
            "Ashar",
            kota.data.jadwal.ashar,
            kabupaten.data.jadwal.ashar
        ]);
        table.add_row(row![
            "Maghrib",
            kota.data.jadwal.maghrib,
            kabupaten.data.jadwal.maghrib
        ]);
        table.add_row(row![
            "Isya",
            kota.data.jadwal.isya,
            kabupaten.data.jadwal.isya
        ]);
        table.printstd();
    } else {
        println!("Nothing Happens");
    }
}