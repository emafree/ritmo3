use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

use crate::error::WebError;
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
        .with_state(state)
}
