use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LookupItem {
    pub id: i64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BookFormData {
    pub name: String,
    pub original_title: Option<String>,
    pub publication_date_year: Option<i32>,
    pub publication_date_month: Option<u8>,
    pub publication_date_day: Option<u8>,
    #[serde(default)]
    pub publication_date_circa: bool,
    pub isbn: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    pub has_cover: bool,
    #[serde(default)]
    pub has_paper: bool,
    pub publisher_id: Option<i64>,
    pub format_id: Option<i64>,
    pub series_id: Option<i64>,
    pub series_index: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContentFormData {
    pub name: String,
    pub original_title: Option<String>,
    pub publication_date_year: Option<i32>,
    pub publication_date_month: Option<u8>,
    pub publication_date_day: Option<u8>,
    #[serde(default)]
    pub publication_date_circa: bool,
    pub notes: Option<String>,
    pub type_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonFormData {
    pub name: String,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names: Option<String>,
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub birth_date_year: Option<i32>,
    pub birth_date_month: Option<u8>,
    pub birth_date_day: Option<u8>,
    #[serde(default)]
    pub birth_date_circa: bool,
    pub death_date_year: Option<i32>,
    pub death_date_month: Option<u8>,
    pub death_date_day: Option<u8>,
    #[serde(default)]
    pub death_date_circa: bool,
    pub biography: Option<String>,
    #[serde(default)]
    pub verified: bool,
}
