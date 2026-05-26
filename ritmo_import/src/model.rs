use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PartialDateInput {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    #[serde(default)]
    pub circa: bool,
}

#[derive(Debug, Deserialize)]
pub struct PersonFile {
    pub person: Vec<PersonInput>,
}

#[derive(Debug, Deserialize)]
pub struct PersonInput {
    pub name: String,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names: Option<String>,
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub birth_date: Option<PartialDateInput>,
    pub death_date: Option<PartialDateInput>,
    pub biography: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub language: Vec<PersonLanguageInput>,
    #[serde(default)]
    pub place: Vec<PersonPlaceInput>,
}

#[derive(Debug, Deserialize)]
pub struct PersonLanguageInput {
    pub iso2: Option<String>,
    pub iso3: Option<String>,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct PersonPlaceInput {
    #[serde(rename = "type")]
    pub place_type: String,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    #[serde(default)]
    pub circa: bool,
    #[serde(default)]
    pub disputed: bool,
}

#[derive(Debug, Deserialize)]
pub struct BookFile {
    pub book: Vec<BookInput>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BookInput {
    pub name: String,
    pub original_title: Option<String>,
    pub format: Option<String>,
    pub publisher: Option<String>,
    pub series: Option<String>,
    pub series_index: Option<i64>,
    pub publication_date: Option<PartialDateInput>,
    pub isbn: Option<String>,
    pub notes: Option<String>,
    #[serde(default)]
    pub has_cover: bool,
    #[serde(default)]
    pub has_paper: bool,
    pub file_link: Option<String>,
    #[serde(default)]
    pub tags: Vec<TagInput>,
    #[serde(default)]
    pub language: Vec<BookLanguageInput>,
    #[serde(default)]
    pub person: Vec<BookPersonInput>,
    #[serde(default)]
    pub content: Vec<ContentInput>,
}

#[derive(Debug, Deserialize)]
pub struct BookLanguageInput {
    pub iso2: Option<String>,
    pub iso3: Option<String>,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct BookPersonInput {
    pub name: String,
    pub role: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ContentInput {
    pub name: String,
    pub original_title: Option<String>,
    #[serde(rename = "type")]
    pub content_type: Option<String>,
    pub genre: Option<String>,
    pub publication_date: Option<PartialDateInput>,
    pub notes: Option<String>,
    #[serde(default)]
    pub language: Vec<ContentLanguageInput>,
    #[serde(default)]
    pub person: Vec<BookPersonInput>,
}

#[derive(Debug, Deserialize)]
pub struct ContentLanguageInput {
    pub iso2: Option<String>,
    pub iso3: Option<String>,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct TagInput {
    pub name: String,
    pub tag_type: String,
}
