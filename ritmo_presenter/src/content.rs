use ritmo_domain::{Content, Language, Tag};

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
    pub genre: Option<String>,
}

pub fn build_content_detail(
    content: Content,
    people_with_roles: Vec<PersonRoleView>,
    tags: Vec<Tag>,
    linked_books: Vec<BookListItem>,
    languages: Vec<Language>,
    genre: Option<String>,
) -> ContentDetail {
    ContentDetail {
        content,
        linked_books,
        people_with_roles,
        tags: tags.into_iter().map(|t| t.name).collect(),
        languages: languages.into_iter().map(|l| l.name).collect(),
        genre,
    }
}
