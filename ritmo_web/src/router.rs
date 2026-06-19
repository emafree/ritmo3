use axum::extract::State;
use axum::response::Html;
use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::error::WebError;
use crate::handlers::{dev, languages, lookups, people_roles, places, tags};
use crate::state::AppState;

async fn index(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let ctx = tera::Context::new();
    let html = state
        .tera
        .render("base.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/dev/widgets", get(dev::widgets))
        // Places widget
        .route("/places/search-panel", get(places::search_panel))
        .route("/places/search", get(places::search))
        .route("/places/{place_id}/edit-row", get(places::edit_row))
        .route("/places", post(places::create))
        .route("/places/{place_id}", put(places::update))
        .route("/{entity_type}/{entity_id}/places", post(places::link))
        .route("/{entity_type}/{entity_id}/places/new", post(places::create_and_link))
        .route(
            "/{entity_type}/{entity_id}/places/{place_id}",
            delete(places::unlink),
        )
        // People+Roles widget
        .route("/people-roles/search-panel", get(people_roles::search_panel))
        .route("/people/search", get(people_roles::person_search))
        .route("/{entity_type}/{entity_id}/people", post(people_roles::link))
        .route("/{entity_type}/{entity_id}/people/new", post(people_roles::create_and_link_person))
        .route(
            "/{entity_type}/{entity_id}/people/{person_id}/roles/{role_id}",
            delete(people_roles::unlink),
        )
        // Tags widget
        .route("/tags/search", get(tags::search))
        .route("/{entity_type}/{entity_id}/tags", post(tags::link))
        .route(
            "/{entity_type}/{entity_id}/tags/{tag_id}",
            delete(tags::unlink),
        )
        // Single lookup widget
        .route("/lookups/{lookup_kind}/search", get(lookups::search))
        .route(
            "/{entity_type}/{entity_id}/lookups/{lookup_kind}",
            post(lookups::set).delete(lookups::clear),
        )
        // Languages widget
        .route("/languages/search-panel", get(languages::search_panel))
        .route("/languages/search", get(languages::lang_search))
        .route("/{entity_type}/{entity_id}/languages", post(languages::link))
        .route(
            "/{entity_type}/{entity_id}/languages/{language_id}/roles/{role_id}",
            delete(languages::unlink),
        )
        .with_state(state)
}
