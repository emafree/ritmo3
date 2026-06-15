use axum::extract::State;
use axum::response::Html;
use ritmo_core::place::{self, PlaceOwner};
use serde::Serialize;
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize)]
struct DateWidgetExample {
    prefix: &'static str,
    label: &'static str,
    year: Option<i32>,
    month: Option<u8>,
    day: Option<u8>,
    circa: bool,
}

pub async fn widgets(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let linked_places = place::list_linked(&state.core, PlaceOwner::Person(1)).await?;
    let places = linked_places
        .into_iter()
        .map(|place| {
            (
                place.place_id,
                place.continent,
                place.country,
                place.city,
                place.circa,
                place.disputed,
                place.place_type_key,
            )
        })
        .collect::<Vec<_>>();

    let mut ctx = Context::new();
    ctx.insert(
        "date_examples",
        &[
            DateWidgetExample {
                prefix: "publication_date",
                label: "Data di pubblicazione",
                year: Some(1984),
                month: Some(6),
                day: Some(8),
                circa: false,
            },
            DateWidgetExample {
                prefix: "birth_date",
                label: "Data di nascita",
                year: Some(1903),
                month: None,
                day: None,
                circa: true,
            },
            DateWidgetExample {
                prefix: "death_date",
                label: "Data di morte",
                year: None,
                month: None,
                day: None,
                circa: false,
            },
        ],
    );
    ctx.insert("entity_type", "people");
    ctx.insert("entity_id", &1_i64);
    ctx.insert("places", &ritmo_presenter::build_place_items(places, "it"));

    let html = state
        .tera
        .render("dev/widgets.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;

    Ok(Html(html))
}

#[cfg(test)]
mod tests {
    use super::widgets;
    use crate::state::{load_tera, AppConfig, AppState};
    use axum::extract::State;
    use ritmo_core::CoreContext;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[tokio::test]
    async fn widgets_page_renders_sample_prefixes() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let state = AppState::new(
            core,
            AppConfig {
                bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 3001),
                database_url: "sqlite::memory:".to_owned(),
            },
            load_tera().unwrap(),
        );

        let html = widgets(State(state)).await.unwrap().0;
        assert!(html.contains("Widget: Data"));
        assert!(html.contains("publication_date_year"));
        assert!(html.contains("birth_date_circa"));
        assert!(html.contains("Widget: Luoghi"));
    }
}
