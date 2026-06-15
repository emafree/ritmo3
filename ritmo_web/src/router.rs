use axum::routing::get;
use axum::Router;

use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "ritmo_web — reset ok" }))
        .with_state(state)
}
