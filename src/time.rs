use std::collections::HashMap;
use chrono;
use chrono::{Datelike, Timelike};

pub fn get_day() -> String {
    chrono::Utc::now().day().to_string()
}

pub fn get_hour() -> String {
    chrono::Utc::now().hour().to_string()
}
pub fn get_month_num() -> String {
    chrono::Utc::now().month().to_string()
}

pub fn get_month_name(hash_map: &HashMap<u32, String>) -> String {
    hash_map.get(&chrono::Utc::now().month()).unwrap().to_owned()
}
pub fn get_year() -> String {
    chrono::Utc::now().year().to_string()
}
pub fn get_time() -> String {
    let mut time = chrono::Utc::now().time().to_string();
    time.truncate(8);
    time
}
pub fn get_date() -> String {
    chrono::Utc::now().date_naive().to_string()
}