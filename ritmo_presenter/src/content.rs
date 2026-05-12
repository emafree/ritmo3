use ritmo_domain::Content;

use crate::{BookListItem, PersonRoleView};

#[derive(Debug, Clone)]
pub struct ContentListItem {
    pub id: i64,
    pub title: String,
    pub authors: Vec<String>,
    pub genre: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContentDetail {
    pub content: Content,
    pub linked_books: Vec<BookListItem>,
    pub people_with_roles: Vec<PersonRoleView>,
    pub tags: Vec<String>,
    pub languages: Vec<String>,
}
