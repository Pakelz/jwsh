#![allow(unused)]

use std::{fs, path::PathBuf};

use chrono::{Datelike, Local};

pub fn time_now_string() -> String {
    let time = Local::now();
    format!("{}-{}-{}", time.year(), time.month(), time.day())
}

pub fn kota_json_path() -> PathBuf {
    #[cfg(target_os = "macos")]
    let base = {
        let home = std::env::var("HOME").expect("HOME not set");
        PathBuf::from(home)
            .join("Library")
            .join("Application Support")
    };

    #[cfg(not(target_os = "macos"))]
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap();
            PathBuf::from(home).join(".config")
        });

    let path = base.join("jwsh").join("kota.json");

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    path
}
