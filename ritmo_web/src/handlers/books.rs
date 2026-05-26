use crate::error::WebError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::Html;
use ritmo_presenter::build_book_list_items;
use ritmo_repository::BookRepository;
use tera::Context;

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let repo = BookRepository::new(&state.repo);
    let rows = repo.list_all_with_authors().await?;
    let books = build_book_list_items(rows);

    let mut ctx = Context::new();
    ctx.insert("books", &books);

    let body = state
        .tera
        .render("books/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, WebError> {
    let mut ctx = Context::new();
    ctx.insert("id", &id);

    let body = state
        .tera
        .render("books/detail.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

pub async fn form(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let ctx = Context::new();

    let body = state
        .tera
        .render("books/form.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}
