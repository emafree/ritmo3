use serde::{Deserialize, Serialize};

fn deserialize_optional_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        s.parse::<i32>().map(Some).map_err(serde::de::Error::custom)
    }
}

fn deserialize_optional_u8<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        s.parse::<u8>().map(Some).map_err(serde::de::Error::custom)
    }
}

fn deserialize_optional_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        s.parse::<i64>().map(Some).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LookupItem {
    pub id: i64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BookFormData {
    pub name: String,
    pub original_title: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_i32")]
    pub publication_date_year: Option<i32>,
    #[serde(deserialize_with = "deserialize_optional_u8")]
    pub publication_date_month: Option<u8>,
    #[serde(deserialize_with = "deserialize_optional_u8")]
    pub publication_date_day: Option<u8>,
    #[serde(default)]
    pub publication_date_circa: bool,
    pub isbn: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    pub has_cover: bool,
    #[serde(default)]
    pub has_paper: bool,
    #[serde(deserialize_with = "deserialize_optional_i64")]
    pub publisher_id: Option<i64>,
    #[serde(deserialize_with = "deserialize_optional_i64")]
    pub format_id: Option<i64>,
    #[serde(deserialize_with = "deserialize_optional_i64")]
    pub series_id: Option<i64>,
    #[serde(deserialize_with = "deserialize_optional_i64")]
    pub series_index: Option<i64>,
    #[serde(default)]
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContentFormData {
    pub name: String,
    pub original_title: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_i32")]
    pub publication_date_year: Option<i32>,
    #[serde(deserialize_with = "deserialize_optional_u8")]
    pub publication_date_month: Option<u8>,
    #[serde(deserialize_with = "deserialize_optional_u8")]
    pub publication_date_day: Option<u8>,
    #[serde(default)]
    pub publication_date_circa: bool,
    pub notes: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_i64")]
    pub type_id: Option<i64>,
    #[serde(default)]
    pub tags: String,
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
    #[serde(deserialize_with = "deserialize_optional_i32")]
    pub birth_date_year: Option<i32>,
    #[serde(deserialize_with = "deserialize_optional_u8")]
    pub birth_date_month: Option<u8>,
    #[serde(deserialize_with = "deserialize_optional_u8")]
    pub birth_date_day: Option<u8>,
    #[serde(default)]
    pub birth_date_circa: bool,
    #[serde(deserialize_with = "deserialize_optional_i32")]
    pub death_date_year: Option<i32>,
    #[serde(deserialize_with = "deserialize_optional_u8")]
    pub death_date_month: Option<u8>,
    #[serde(deserialize_with = "deserialize_optional_u8")]
    pub death_date_day: Option<u8>,
    #[serde(default)]
    pub death_date_circa: bool,
    pub biography: Option<String>,
    #[serde(default)]
    pub verified: bool,
}
