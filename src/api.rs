#![allow(unused)]
use crate::{
    helper::{self, kota_json_path, time_now_string},
    models::{Jadwal, Response},
};

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.myquran.com/v2/sholat/kota/semua";
    let response: Response = reqwest::blocking::get(url)?.json()?;
    let json = serde_json::to_string_pretty(&response)?;
    std::fs::write(helper::kota_json_path(), json)?;
    Ok(())
}

pub fn find_city(input: &str) -> Result<(Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(kota_json_path())?;
    let response: Response = serde_json::from_str(&file)?;

    Ok(response.get_id(input))
}

pub fn get_jadwal(jadwal: Vec<String>) -> Result<Vec<Jadwal>, Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    for jw in jadwal {
        let url = format!(
            "https://api.myquran.com/v2/sholat/jadwal/{}/{}",
            jw,
            time_now_string()
        );

        let response: Jadwal = reqwest::blocking::get(url)?.json()?;
        data.push(response);
    }
    Ok(data)
}
