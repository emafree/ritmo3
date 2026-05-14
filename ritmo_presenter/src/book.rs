use ritmo_domain::{Book, Tag};

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
    pub format: Option<String>,
    pub series: Option<String>,
}

pub fn build_book_detail(
    book: Book,
    people_with_roles: Vec<PersonRoleView>,
    tags: Vec<Tag>,
    linked_contents: Vec<ContentListItem>,
    format: Option<String>,
    series: Option<String>,
) -> BookDetail {
    BookDetail {
        book,
        linked_contents,
        people_with_roles,
        tags: tags.into_iter().map(|t| t.name).collect(),
        format,
        series,
    }
}
