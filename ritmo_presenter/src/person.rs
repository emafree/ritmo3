use ritmo_domain::{Person, Role};
use serde::Serialize;

use crate::{BookListItem, ContentListItem};

#[derive(Debug, Clone, Serialize)]
pub struct PersonListItem {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub birth_year: Option<i64>,
    pub death_year: Option<i64>,
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

pub fn build_person_role_view(person: &Person, role: &Role) -> PersonRoleView {
    PersonRoleView {
        person_id: person.id,
        name: person
            .display_name
            .clone()
            .unwrap_or_else(|| person.name.clone()),
        role: role.i18n_key.clone(),
    }
}

pub fn build_person_role_views(people_roles: &[(Person, Role)]) -> Vec<PersonRoleView> {
    people_roles
        .iter()
        .map(|(person, role)| build_person_role_view(person, role))
        .collect()
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
