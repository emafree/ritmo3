use crate::PlaceItem;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PublisherListItem {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PublisherDetail {
    pub id: i64,
    pub name: String,
    pub country: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub places: Vec<PlaceItem>,
}

pub fn build_publisher_list_items(rows: Vec<(i64, String)>) -> Vec<PublisherListItem> {
    rows.into_iter()
        .map(|(id, name)| PublisherListItem { id, name })
        .collect()
}

#[allow(clippy::type_complexity)]
pub fn build_publisher_detail(
    id: i64,
    name: String,
    country: Option<String>,
    website: Option<String>,
    notes: Option<String>,
    places: Vec<(
        i64,
        Option<String>,
        Option<String>,
        Option<String>,
        bool,
        bool,
        String,
        String,
    )>,
) -> PublisherDetail {
    PublisherDetail {
        id,
        name,
        country,
        website,
        notes,
        places: places
            .into_iter()
            .map(
                |(
                    place_id,
                    continent,
                    country,
                    city,
                    circa,
                    disputed,
                    place_type_key,
                    place_type_label,
                )| PlaceItem {
                    place_id,
                    continent,
                    country,
                    city,
                    circa,
                    disputed,
                    place_type_key,
                    place_type_label,
                },
            )
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::build_publisher_detail;

    #[test]
    fn build_publisher_detail_maps_places() {
        let detail = build_publisher_detail(
            1,
            "Editore".to_owned(),
            Some("Italia".to_owned()),
            Some("https://example.org".to_owned()),
            Some("Note".to_owned()),
            vec![(
                10,
                Some("Europa".to_owned()),
                Some("Italia".to_owned()),
                Some("Roma".to_owned()),
                false,
                true,
                "activity".to_owned(),
                "Luogo di attività".to_owned(),
            )],
        );

        assert_eq!(detail.id, 1);
        assert_eq!(detail.name, "Editore");
        assert_eq!(detail.places.len(), 1);
        assert_eq!(detail.places[0].place_type_key, "activity");
        assert_eq!(detail.places[0].place_type_label, "Luogo di attività");
    }
}
