use axum::extract::State;
use axum::response::Html;
use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::error::WebError;
use crate::handlers::{dev, places};
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
        .route("/places/search-panel", get(places::search_panel))
        .route("/places/search", get(places::search))
        .route("/places/{place_id}/edit-row", get(places::edit_row))
        .route("/places", post(places::create))
        .route("/places/{place_id}", put(places::update))
        .route("/{entity_type}/{entity_id}/places", post(places::link))
        .route(
            "/{entity_type}/{entity_id}/places/{place_id}",
            delete(places::unlink),
        )
        .with_state(state)
}
