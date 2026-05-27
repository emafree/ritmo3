use crate::{
    format_language_name, format_partial_date, LanguageItem, LinkedItem, PersonWithRole, TagItem,
};
use ritmo_domain::PartialDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BookListItem {
    pub id: i64,
    pub title: String,
    pub authors: Vec<String>,
    pub format: Option<String>,
    pub series: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct BookDetail {
    pub id: i64,
    pub name: String,
    pub has_contents: bool,
    pub original_title: Option<String>,
    pub publisher: Option<String>,
    pub format: Option<String>,
    pub series: Option<String>,
    pub series_index: Option<i64>,
    pub publication_date: Option<String>,
    pub isbn: Option<String>,
    pub notes: Option<String>,
    pub has_cover: bool,
    pub has_paper: bool,
    pub contents: Vec<LinkedItem>,
    pub people: Vec<PersonWithRole>,
    pub tags: Vec<TagItem>,
    pub languages: Vec<LanguageItem>,
}

pub fn build_book_detail(
    id: i64,
    name: String,
    has_contents: bool,
    original_title: Option<String>,
    publisher: Option<String>,
    format: Option<String>,
    series: Option<String>,
    series_index: Option<i64>,
    publication_date: Option<PartialDate>,
    isbn: Option<String>,
    notes: Option<String>,
    has_cover: bool,
    has_paper: bool,
    contents: Vec<(i64, String)>,
    people: Vec<(i64, String, String)>,
    tags: Vec<(String, String)>,
    languages: Vec<(String, Option<String>, String)>,
) -> BookDetail {
    BookDetail {
        id,
        name,
        has_contents,
        original_title,
        publisher,
        format,
        series,
        series_index,
        publication_date: format_partial_date(publication_date),
        isbn,
        notes,
        has_cover,
        has_paper,
        contents: contents
            .into_iter()
            .map(|(id, name)| LinkedItem { id, name })
            .collect(),
        people: people
            .into_iter()
            .map(|(id, name, role)| PersonWithRole { id, name, role })
            .collect(),
        tags: tags
            .into_iter()
            .map(|(name, tag_type)| TagItem { name, tag_type })
            .collect(),
        languages: languages
            .into_iter()
            .map(|(name, iso_code_2char, role)| LanguageItem {
                name: format_language_name(name, iso_code_2char),
                role,
            })
            .collect(),
    }
}

pub fn build_book_list_items(
    rows: Vec<(i64, String, Vec<String>, Option<String>, Option<String>)>,
) -> Vec<BookListItem> {
    rows.into_iter()
        .map(|(id, title, authors, format, series)| BookListItem {
            id,
            title,
            authors,
            format,
            series,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::build_book_detail;
    use ritmo_domain::PartialDate;

    #[test]
    fn build_book_detail_formats_nested_fields() {
        let detail = build_book_detail(
            1,
            "Libro".to_owned(),
            true,
            Some("Original".to_owned()),
            Some("Editore".to_owned()),
            Some("paperback".to_owned()),
            Some("Saga".to_owned()),
            Some(2),
            Some(PartialDate {
                year: Some(1984),
                month: Some(3),
                day: Some(15),
                circa: true,
            }),
            Some("123".to_owned()),
            Some("Note".to_owned()),
            true,
            false,
            vec![(10, "Capitolo".to_owned())],
            vec![(20, "Autore".to_owned(), "author".to_owned())],
            vec![("Fantascienza".to_owned(), "genre".to_owned())],
            vec![(
                "Italian".to_owned(),
                Some("it".to_owned()),
                "translation".to_owned(),
            )],
        );

        assert_eq!(detail.publication_date.as_deref(), Some("~15 marzo 1984"));
        assert!(detail.has_contents);
        assert_eq!(detail.contents[0].name, "Capitolo");
        assert_eq!(detail.people[0].role, "author");
        assert_eq!(detail.tags[0].tag_type, "genre");
        assert_eq!(detail.languages[0].name, "Italian (it)");
    }
}
