use crate::I18nDisplayable;
use ritmo_domain::{ContentType, Format, PartialDate, Publisher, Series};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LinkedItem {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LinkedItemWithRole {
    pub id: i64,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PersonWithRole {
    pub id: i64,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TagItem {
    pub name: String,
    pub tag_type: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LanguageItem {
    pub name: String,
    pub role: String,
}

/// Item used by the People+Roles widget to represent a linked (person, role) pair.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PeopleRoleItem {
    pub person_id: i64,
    pub person_name: String,
    pub role_id: i64,
    pub role_key: String,
}

/// Tag badge used by the Tag widget.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TagBadge {
    pub tag_id: i64,
    pub name: String,
    pub tag_type: String,
}

/// Item used by the Language widget to represent a linked (language, role) pair.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LangWidgetItem {
    pub language_id: i64,
    pub official_name: String,
    pub role_id: i64,
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LookupSearchResultItem {
    pub id: i64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
pub struct LookupWidgetState {
    pub current: Option<LookupSearchResultItem>,
}

pub fn build_people_role_items(
    pairs: Vec<(i64, String, i64, String)>,
) -> Vec<PeopleRoleItem> {
    pairs
        .into_iter()
        .map(|(person_id, person_name, role_id, role_key)| PeopleRoleItem {
            person_id,
            person_name,
            role_id,
            role_key,
        })
        .collect()
}

pub fn build_tag_badges(tags: Vec<(i64, String, String)>) -> Vec<TagBadge> {
    tags.into_iter()
        .map(|(tag_id, name, tag_type)| TagBadge {
            tag_id,
            name,
            tag_type,
        })
        .collect()
}

pub fn build_lang_widget_items(
    langs: Vec<(i64, String, i64, String)>,
) -> Vec<LangWidgetItem> {
    langs
        .into_iter()
        .map(|(language_id, official_name, role_id, role_name)| LangWidgetItem {
            language_id,
            official_name,
            role_id,
            role_name,
        })
        .collect()
}

pub fn build_publisher_lookup_item(item: Publisher) -> LookupSearchResultItem {
    LookupSearchResultItem {
        id: item.id,
        label: item.name,
    }
}

pub fn build_series_lookup_item(item: Series) -> LookupSearchResultItem {
    LookupSearchResultItem {
        id: item.id,
        label: item.name,
    }
}

pub fn build_format_lookup_item(item: Format, locale: &str) -> LookupSearchResultItem {
    LookupSearchResultItem {
        id: item.id,
        label: item.display_name(locale),
    }
}

pub fn build_content_type_lookup_item(item: ContentType, locale: &str) -> LookupSearchResultItem {
    let label = item.display_name(locale);
    LookupSearchResultItem {
        id: item.id,
        label: if label.is_empty() { item.i18n_key } else { label },
    }
}

pub fn build_publisher_lookup_items(items: Vec<Publisher>) -> Vec<LookupSearchResultItem> {
    items.into_iter().map(build_publisher_lookup_item).collect()
}

pub fn build_series_lookup_items(items: Vec<Series>) -> Vec<LookupSearchResultItem> {
    items.into_iter().map(build_series_lookup_item).collect()
}

pub fn build_format_lookup_items(items: Vec<Format>, locale: &str) -> Vec<LookupSearchResultItem> {
    items
        .into_iter()
        .map(|item| build_format_lookup_item(item, locale))
        .collect()
}

pub fn build_content_type_lookup_items(
    items: Vec<ContentType>,
    locale: &str,
) -> Vec<LookupSearchResultItem> {
    items
        .into_iter()
        .map(|item| build_content_type_lookup_item(item, locale))
        .collect()
}

pub fn build_lookup_widget_state(current: Option<LookupSearchResultItem>) -> LookupWidgetState {
    LookupWidgetState { current }
}

pub fn format_partial_date(date: Option<PartialDate>) -> Option<String> {
    let date = date?;
    let mut parts = Vec::new();

    if let Some(day) = date.day {
        parts.push(day.to_string());
    }

    if let Some(month) = date.month {
        parts.push(month_name(month).to_owned());
    }

    if let Some(year) = date.year {
        parts.push(year.to_string());
    }

    if parts.is_empty() {
        return None;
    }

    let formatted = parts.join(" ");
    Some(if date.circa {
        format!("~{formatted}")
    } else {
        formatted
    })
}

pub fn format_language_name(name: String, iso_code_2char: Option<String>) -> String {
    match iso_code_2char {
        Some(code) if !code.is_empty() => format!("{name} ({code})"),
        _ => name,
    }
}

pub fn build_place_display(
    continent: Option<String>,
    country: Option<String>,
    city: Option<String>,
) -> String {
    let mut parts = Vec::new();

    if let Some(city) = city.filter(|value| !value.is_empty()) {
        parts.push(city);
    }
    if let Some(country) = country.filter(|value| !value.is_empty()) {
        parts.push(country);
    }
    if let Some(continent) = continent.filter(|value| !value.is_empty()) {
        parts.push(continent);
    }

    if parts.is_empty() {
        "—".to_owned()
    } else {
        parts.join(", ")
    }
}

fn month_name(month: u8) -> &'static str {
    match month {
        1 => "gennaio",
        2 => "febbraio",
        3 => "marzo",
        4 => "aprile",
        5 => "maggio",
        6 => "giugno",
        7 => "luglio",
        8 => "agosto",
        9 => "settembre",
        10 => "ottobre",
        11 => "novembre",
        12 => "dicembre",
        _ => "mese sconosciuto",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_content_type_lookup_item, build_lookup_widget_state, build_place_display,
        format_language_name, format_partial_date,
    };
    use ritmo_domain::{ContentType, PartialDate};

    #[test]
    fn format_partial_date_uses_available_parts_and_circa_prefix() {
        assert_eq!(
            format_partial_date(Some(PartialDate {
                year: Some(1984),
                month: Some(3),
                day: Some(15),
                circa: true,
            })),
            Some("~15 marzo 1984".to_owned())
        );
        assert_eq!(
            format_partial_date(Some(PartialDate {
                year: Some(1984),
                month: Some(3),
                day: None,
                circa: false,
            })),
            Some("marzo 1984".to_owned())
        );
        assert_eq!(
            format_partial_date(Some(PartialDate {
                year: Some(1984),
                month: None,
                day: None,
                circa: false,
            })),
            Some("1984".to_owned())
        );
    }

    #[test]
    fn place_and_language_helpers_format_display_values() {
        assert_eq!(
            build_place_display(
                Some("Europa".to_owned()),
                Some("Italia".to_owned()),
                Some("Roma".to_owned())
            ),
            "Roma, Italia, Europa"
        );
        assert_eq!(
            format_language_name("Italian".to_owned(), Some("it".to_owned())),
            "Italian (it)"
        );
    }

    #[test]
    fn content_type_lookup_item_falls_back_to_key_for_missing_translation() {
        let item = build_content_type_lookup_item(
            ContentType {
                id: 7,
                i18n_key: "inline_type".to_owned(),
            },
            "it",
        );
        let state = build_lookup_widget_state(Some(item.clone()));

        assert_eq!(item.label, "inline_type");
        assert_eq!(state.current, Some(item));
    }
}
