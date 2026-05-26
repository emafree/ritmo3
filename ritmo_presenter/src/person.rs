use crate::{
    build_place_display, format_language_name, format_partial_date, LanguageItem,
    LinkedItemWithRole, PlaceItem,
};
use ritmo_domain::PartialDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PersonListItem {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub birth_year: Option<i64>,
    pub death_year: Option<i64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PersonDetail {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub biography: Option<String>,
    pub aliases: Vec<String>,
    pub places: Vec<PlaceItem>,
    pub languages: Vec<LanguageItem>,
    pub books: Vec<LinkedItemWithRole>,
    pub contents: Vec<LinkedItemWithRole>,
}

pub fn build_person_detail(
    id: i64,
    name: String,
    display_name: Option<String>,
    birth_date: Option<PartialDate>,
    death_date: Option<PartialDate>,
    biography: Option<String>,
    aliases: Vec<String>,
    places: Vec<(String, Option<String>, Option<String>, Option<String>)>,
    languages: Vec<(String, Option<String>, String)>,
    books: Vec<(i64, String, String)>,
    contents: Vec<(i64, String, String)>,
) -> PersonDetail {
    PersonDetail {
        id,
        name,
        display_name,
        birth_date: format_partial_date(birth_date),
        death_date: format_partial_date(death_date),
        biography,
        aliases,
        places: places
            .into_iter()
            .map(|(place_type, continent, country, city)| PlaceItem {
                place_type,
                display: build_place_display(continent, country, city),
            })
            .collect(),
        languages: languages
            .into_iter()
            .map(|(name, iso_code_2char, role)| LanguageItem {
                name: format_language_name(name, iso_code_2char),
                role,
            })
            .collect(),
        books: books
            .into_iter()
            .map(|(id, name, role)| LinkedItemWithRole { id, name, role })
            .collect(),
        contents: contents
            .into_iter()
            .map(|(id, name, role)| LinkedItemWithRole { id, name, role })
            .collect(),
    }
}

pub fn build_person_list_items(
    rows: Vec<(i64, String, Option<String>, Option<i64>, Option<i64>)>,
) -> Vec<PersonListItem> {
    rows.into_iter()
        .map(
            |(id, name, display_name, birth_year, death_year)| PersonListItem {
                id,
                name,
                display_name,
                birth_year,
                death_year,
            },
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::build_person_detail;
    use ritmo_domain::PartialDate;

    #[test]
    fn build_person_detail_formats_dates_and_places() {
        let detail = build_person_detail(
            1,
            "Ursula K. Le Guin".to_owned(),
            Some("Le Guin".to_owned()),
            Some(PartialDate {
                year: Some(1929),
                month: Some(10),
                day: Some(21),
                circa: false,
            }),
            Some(PartialDate {
                year: Some(2018),
                month: None,
                day: None,
                circa: false,
            }),
            Some("Biografia".to_owned()),
            vec!["Alias".to_owned()],
            vec![(
                "birth".to_owned(),
                Some("Nord America".to_owned()),
                Some("USA".to_owned()),
                Some("Berkeley".to_owned()),
            )],
            vec![(
                "English".to_owned(),
                Some("en".to_owned()),
                "native".to_owned(),
            )],
            vec![(
                10,
                "The Left Hand of Darkness".to_owned(),
                "author".to_owned(),
            )],
            vec![(
                20,
                "The Ones Who Walk Away from Omelas".to_owned(),
                "author".to_owned(),
            )],
        );

        assert_eq!(detail.birth_date.as_deref(), Some("21 ottobre 1929"));
        assert_eq!(detail.death_date.as_deref(), Some("2018"));
        assert_eq!(detail.places[0].display, "Berkeley, USA, Nord America");
        assert_eq!(detail.languages[0].name, "English (en)");
        assert_eq!(detail.books[0].role, "author");
        assert_eq!(
            detail.contents[0].name,
            "The Ones Who Walk Away from Omelas"
        );
    }
}
