use axum::extract::State;
use axum::response::Html;
use ritmo_core::place::{self, PlaceOwner};
use ritmo_core::{book, content, lookup};
use serde::Serialize;
use tera::Context;

use crate::error::WebError;
use crate::handlers::lookups::render_lookup_widget;
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
    // Places (entity: people/1)
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

    // People+Roles (entity: books/1)
    let pr_pairs = book::list_people_with_roles(&state.core, 1)
        .await
        .unwrap_or_default();
    let pr_people_roles = ritmo_presenter::build_people_role_items(
        pr_pairs
            .into_iter()
            .map(|(person, role)| (person.id, person.name, role.id, role.i18n_key))
            .collect(),
    );

    // Tags (entity: books/1)
    let tag_list = book::list_tags(&state.core, 1).await.unwrap_or_default();
    let tag_items = ritmo_presenter::build_tag_badges(
        tag_list
            .into_iter()
            .map(|t| (t.id, t.name, t.tag_type))
            .collect(),
    );

    // Languages (entity: contents/1)
    let lang_pairs = content::list_languages_with_roles(&state.core, 1)
        .await
        .unwrap_or_default();
    let lang_items = ritmo_presenter::build_lang_widget_items(lang_pairs);
    let publisher_lookup = render_lookup_widget(&state, "books", 1, lookup::LookupKind::Publisher)
        .await?
        .0;
    let series_lookup = render_lookup_widget(&state, "books", 1, lookup::LookupKind::Series)
        .await?
        .0;
    let format_lookup = render_lookup_widget(&state, "books", 1, lookup::LookupKind::Format)
        .await?
        .0;
    let type_lookup = render_lookup_widget(&state, "contents", 1, lookup::LookupKind::Type)
        .await?
        .0;

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
    ctx.insert("pr_entity_type", "books");
    ctx.insert("pr_entity_id", &1_i64);
    ctx.insert("pr_people_roles", &pr_people_roles);
    ctx.insert("tag_entity_type", "books");
    ctx.insert("tag_entity_id", &1_i64);
    ctx.insert("tag_items", &tag_items);
    ctx.insert("lang_entity_type", "contents");
    ctx.insert("lang_entity_id", &1_i64);
    ctx.insert("lang_items", &lang_items);
    ctx.insert("publisher_lookup_widget", &publisher_lookup);
    ctx.insert("series_lookup_widget", &series_lookup);
    ctx.insert("format_lookup_widget", &format_lookup);
    ctx.insert("type_lookup_widget", &type_lookup);

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
    use ritmo_domain::{Book, Content};
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

        ritmo_core::book::create(
            &state.core,
            &Book {
                id: 0,
                title: "Libro widget".to_owned(),
                original_title: None,
                publisher_id: None,
                format_id: None,
                series_id: None,
                series_index: None,
                isbn: None,
                publication_year: None,
                notes: None,
                has_cover: false,
                has_paper: false,
            },
        )
        .await
        .unwrap();
        ritmo_core::content::create(
            &state.core,
            &Content {
                id: 0,
                title: "Contenuto widget".to_owned(),
                original_title: None,
                type_id: None,
                publication_year: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let html = widgets(State(state)).await.unwrap().0;
        assert!(html.contains("Widget: Data"));
        assert!(html.contains("publication_date_year"));
        assert!(html.contains("birth_date_circa"));
        assert!(html.contains("Widget: Luoghi"));
        assert!(html.contains("Widget: Lookup Publisher"));
        assert!(html.contains("Widget: Lookup Type"));
    }
}
