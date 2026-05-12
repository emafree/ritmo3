use ritmo_domain::Person;

use crate::{BookListItem, ContentListItem};

#[derive(Debug, Clone)]
pub struct PersonListItem {
    pub id: i64,
    pub name: String,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PersonDetail {
    pub person: Person,
    pub aliases: Vec<String>,
    pub places: Vec<PlaceView>,
    pub linked_books: Vec<BookListItem>,
    pub linked_contents: Vec<ContentListItem>,
}

#[derive(Debug, Clone)]
pub struct PlaceView {
    pub id: i64,
    pub name: String,
    pub place_type: String,
}

#[derive(Debug, Clone)]
pub struct PersonRoleView {
    pub person_id: i64,
    pub name: String,
    pub role: String,
}
