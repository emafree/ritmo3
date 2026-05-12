use ritmo_domain::Book;

use crate::{ContentListItem, PersonRoleView};

#[derive(Debug, Clone)]
pub struct BookListItem {
    pub id: i64,
    pub title: String,
    pub authors: Vec<String>,
    pub format: Option<String>,
    pub series: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BookDetail {
    pub book: Book,
    pub linked_contents: Vec<ContentListItem>,
    pub people_with_roles: Vec<PersonRoleView>,
    pub tags: Vec<String>,
}
