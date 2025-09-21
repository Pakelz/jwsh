#![allow(unused)]

use chrono::{Datelike, Local};

pub fn time_now_string() -> String {
    let time = Local::now();
    format!("{}-{}-{}", time.year(), time.month(), time.day())
}
