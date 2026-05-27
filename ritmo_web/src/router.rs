use axum::routing::get;
use axum::Router;

use crate::handlers::{books, contents, lookups, people};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(books::list))
        .route("/books", get(books::list).post(books::create))
        .route("/books/new", get(books::form))
        .route("/books/{id}", get(books::detail).post(books::save).delete(books::delete))
        .route("/contents", get(contents::list).post(contents::create))
        .route("/contents/new", get(contents::form))
        .route(
            "/contents/{id}",
            get(contents::detail)
                .post(contents::save)
                .delete(contents::delete),
        )
        .route("/people", get(people::list).post(people::create))
        .route("/people/new", get(people::form))
        .route("/people/{id}", get(people::detail).post(people::save).delete(people::delete))
        .route("/lookups/tags", get(lookups::tags))
        .route("/lookups/publishers", get(lookups::publishers))
        .route("/lookups/series", get(lookups::series))
        .route("/lookups/formats", get(lookups::formats))
        .route("/lookups/types", get(lookups::types))
        .route("/lookups/roles", get(lookups::roles))
        .route("/lookups/languages", get(lookups::languages))
        .route("/publishers/{id}", axum::routing::delete(lookups::delete_publisher))
        .route("/series/{id}", axum::routing::delete(lookups::delete_series))
        .route("/formats/{id}", axum::routing::delete(lookups::delete_format))
        .route("/roles/{id}", axum::routing::delete(lookups::delete_role))
        .route("/languages/{id}", axum::routing::delete(lookups::delete_language))
        .route("/tags/{id}", axum::routing::delete(lookups::delete_tag))
        .with_state(state)
}
