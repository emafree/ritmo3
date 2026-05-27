use crate::{
    format_language_name, format_partial_date, LanguageItem, LinkedItem, PersonWithRole, TagItem,
};
use ritmo_domain::PartialDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ContentListItem {
    pub id: i64,
    pub title: String,
    pub name: String,
    pub type_key: Option<String>,
    pub original_language: Option<String>,
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContentDetail {
    pub id: i64,
    pub name: String,
    pub has_author: bool,
    pub original_title: Option<String>,
    pub content_type: Option<String>,
    pub publication_date: Option<String>,
    pub notes: Option<String>,
    pub books: Vec<LinkedItem>,
    pub people: Vec<PersonWithRole>,
    pub tags: Vec<TagItem>,
    pub languages: Vec<LanguageItem>,
}

pub fn build_content_detail(
    id: i64,
    name: String,
    has_author: bool,
    original_title: Option<String>,
    content_type: Option<String>,
    publication_date: Option<PartialDate>,
    notes: Option<String>,
    books: Vec<(i64, String)>,
    people: Vec<(i64, String, String)>,
    tags: Vec<(String, String)>,
    languages: Vec<(String, Option<String>, String)>,
) -> ContentDetail {
    ContentDetail {
        id,
        name,
        has_author,
        original_title,
        content_type,
        publication_date: format_partial_date(publication_date),
        notes,
        books: books
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

pub fn build_content_list_items(
    rows: Vec<(i64, String, Option<String>, Option<String>, Vec<String>)>,
) -> Vec<ContentListItem> {
    rows.into_iter()
        .map(
            |(id, name, type_key, original_language, authors)| ContentListItem {
                id,
                title: name.clone(),
                name,
                type_key,
                original_language,
                authors,
            },
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{build_content_detail, build_content_list_items};
    use ritmo_domain::PartialDate;

    #[test]
    fn build_content_detail_formats_lists() {
        let detail = build_content_detail(
            1,
            "Racconto".to_owned(),
            true,
            Some("Story".to_owned()),
            Some("short_story".to_owned()),
            Some(PartialDate {
                year: Some(1952),
                month: Some(7),
                day: None,
                circa: false,
            }),
            Some("Note".to_owned()),
            vec![(10, "Antologia".to_owned())],
            vec![(20, "Autrice".to_owned(), "author".to_owned())],
            vec![("Noir".to_owned(), "mood".to_owned())],
            vec![(
                "English".to_owned(),
                Some("en".to_owned()),
                "original".to_owned(),
            )],
        );

        assert_eq!(detail.publication_date.as_deref(), Some("luglio 1952"));
        assert!(detail.has_author);
        assert_eq!(detail.books[0].name, "Antologia");
        assert_eq!(detail.people[0].role, "author");
        assert_eq!(detail.tags[0].tag_type, "mood");
        assert_eq!(detail.languages[0].name, "English (en)");
    }

    #[test]
    fn build_content_list_items_sets_title() {
        let rows = vec![(
            1,
            "Racconto".to_owned(),
            Some("short_story".to_owned()),
            Some("Italiano".to_owned()),
            vec!["Autore".to_owned()],
        )];

        let items = build_content_list_items(rows);

        assert_eq!(items[0].name, "Racconto");
        assert_eq!(items[0].title, "Racconto");
    }
}
