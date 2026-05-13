pub mod filter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialDate {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub circa: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alias {
    pub id: i64,
    pub alternative_name: String,
    pub person_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    pub id: i64,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub circa: bool,
    pub disputed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceType {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub id: i64,
    pub iso_639_2: Option<String>,
    pub iso_639_3: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
    pub id: i64,
    pub i18n_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub i18n_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub i18n_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publisher {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names: Option<String>,
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub birth_date: Option<PartialDate>,
    pub death_date: Option<PartialDate>,
    pub biography: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: i64,
    pub title: String,
    pub publication_year: Option<PartialDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub isbn: Option<String>,
    pub publication_year: Option<PartialDate>,
    pub notes: Option<String>,
}
