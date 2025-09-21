#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Kota {
    id: String,
    lokasi: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    status: bool,
    data: Vec<Kota>,
}

impl Response {
    pub fn get_id(self, input: &str) -> (Vec<String>, Vec<String>) {
        let mut ids = Vec::new();
        let mut lokasi = Vec::new();
        for id in self.data {
            if id
                .lokasi
                .to_lowercase()
                .contains(input.to_lowercase().as_str())
            {
                ids.push(id.id);
                lokasi.push(id.lokasi);
            }
            // ids.push(id.id);
            // lokasi.push(id.lokasi);
        }
        (ids, lokasi)
    }
}

#[derive(Debug, Deserialize)]
pub struct Sholat {
    pub subuh: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String,
}

#[derive(Debug, Deserialize)]
pub struct Jadwal {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub lokasi: String,
    pub jadwal: Sholat,
}