use ritmo_domain::PartialDate;
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PlaceItem {
    pub place_type: String,
    pub display: String,
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
    use super::{build_place_display, format_language_name, format_partial_date};
    use ritmo_domain::PartialDate;

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
}
