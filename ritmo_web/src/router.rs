use axum::routing::get;
use axum::Router;

use crate::handlers::{books, contents, lookups, people};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(books::list))
        .route("/books", get(books::list).post(books::create))
        .route("/books/new", get(books::form))
        .route("/books/{id}", get(books::detail).post(books::save))
        .route("/contents", get(contents::list).post(contents::create))
        .route("/contents/new", get(contents::form))
        .route("/contents/{id}", get(contents::detail).post(contents::save))
        .route("/people", get(people::list).post(people::create))
        .route("/people/new", get(people::form))
        .route("/people/{id}", get(people::detail).post(people::save))
        .route("/lookups/tags", get(lookups::tags))
        .route("/lookups/publishers", get(lookups::publishers))
        .route("/lookups/series", get(lookups::series))
        .route("/lookups/formats", get(lookups::formats))
        .route("/lookups/types", get(lookups::types))
        .route("/lookups/roles", get(lookups::roles))
        .with_state(state)
}
