use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Object {
    id: u32,
    name: String,
    published_year: NaiveDate,
    category_id: u32,
    lang_id: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DBBook {
    inner: Object,
    pages_number: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Book {
    id: u32,
    name: String,
    published_year: NaiveDate,
    category: String,
    lang: String,
    pages: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Music {
    id: u32,
    name: String,
    published_year: NaiveDate,
    category: String,
    lang: String,
    duration: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VideoGame {
    id: u32,
    name: String,
    published_year: NaiveDate,
    category: String,
    lang: String,
    supports_console_controller: bool,
    pegi: String,
}
