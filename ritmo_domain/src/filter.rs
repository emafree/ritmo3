use serde::{Deserialize, Serialize};
use crate::PartialDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Contains,
    Equals,
    Between,
    Before,
    After,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterValue {
    Text(String),
    Id(i64),
    Date(PartialDate),
    DateRange(PartialDate, PartialDate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterField {
    // Book
    BookTitle,
    BookIsbn,
    BookFormat,
    BookSeries,
    BookPublisher,
    BookPublicationDate,
    BookTag,
    BookLanguage { role_id: Option<i64> },

    // Content
    ContentTitle,
    ContentGenre,
    ContentPublicationDate,
    ContentTag,
    ContentLanguage { role_id: Option<i64> },

    // Person in relazione
    PersonName,
    PersonRole { role_id: Option<i64> },
    PersonLanguage { role_id: Option<i64> },
    PersonPlace { place_type_id: Option<i64> },
    PersonBirthDate,
    PersonDeathDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub field: FilterField,
    pub operator: FilterOperator,
    pub values: Vec<FilterValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSet {
    pub id: i64,
    pub name: String,
    pub active: bool,
    pub operator: LogicalOperator,
    pub filters: Vec<Filter>,
}
