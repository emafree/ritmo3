use serde::Serialize;

pub type PlaceRow = (
    i64,
    Option<String>,
    Option<String>,
    Option<String>,
    bool,
    bool,
    String,
);

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PlaceItem {
    pub place_id: i64,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub circa: bool,
    pub disputed: bool,
    pub place_type_key: String,
    pub place_type_label: String,
}

pub fn build_place_items(rows: Vec<PlaceRow>, lang: &str) -> Vec<PlaceItem> {
    rows.into_iter()
        .map(
            |(place_id, continent, country, city, circa, disputed, place_type_key)| PlaceItem {
                place_id,
                continent,
                country,
                city,
                circa,
                disputed,
                place_type_label: place_type_label(lang, &place_type_key),
                place_type_key,
            },
        )
        .collect()
}

fn place_type_label(lang: &str, key: &str) -> String {
    match lang {
        "it" => match key {
            "birth" => "Luogo di nascita",
            "death" => "Luogo di morte",
            "activity" => "Luogo di attività",
            "residence" => "Residenza",
            "other" => "Altro",
            _ => key,
        },
        "fr" => match key {
            "birth" => "Lieu de naissance",
            "death" => "Lieu de décès",
            "activity" => "Lieu d'activité",
            "residence" => "Résidence",
            "other" => "Autre",
            _ => key,
        },
        "de" => match key {
            "birth" => "Geburtsort",
            "death" => "Sterbeort",
            "activity" => "Wirkungsort",
            "residence" => "Wohnort",
            "other" => "Andere",
            _ => key,
        },
        _ => match key {
            "birth" => "Place of birth",
            "death" => "Place of death",
            "activity" => "Place of activity",
            "residence" => "Residence",
            "other" => "Other",
            _ => key,
        },
    }
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::build_place_items;

    #[test]
    fn build_place_items_maps_place_and_type_label() {
        let items = build_place_items(
            vec![(
                10,
                Some("Europa".to_owned()),
                Some("Italia".to_owned()),
                Some("Roma".to_owned()),
                true,
                false,
                "birth".to_owned(),
            )],
            "it",
        );

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].place_id, 10);
        assert_eq!(items[0].city.as_deref(), Some("Roma"));
        assert!(items[0].circa);
        assert_eq!(items[0].place_type_key, "birth");
        assert_eq!(items[0].place_type_label, "Luogo di nascita");
    }
}
