use ritmo_domain::{Content, Language, Tag};
use serde::Serialize;

use crate::{BookListItem, PersonRoleView};

#[derive(Debug, Clone, Serialize)]
pub struct ContentListItem {
    pub id: i64,
    pub name: String,
    pub type_key: Option<String>,
    pub original_language: Option<String>,
    pub authors: Vec<String>,
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

pub fn build_content_list_items(
    rows: Vec<(i64, String, Option<String>, Option<String>, Vec<String>)>,
) -> Vec<ContentListItem> {
    rows.into_iter()
        .map(
            |(id, name, type_key, original_language, authors)| ContentListItem {
                id,
                name,
                type_key,
                original_language,
                authors,
            },
        )
        .collect()
}
