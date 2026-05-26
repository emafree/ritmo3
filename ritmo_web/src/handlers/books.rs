use crate::error::WebError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::Html;
use ritmo_presenter::{build_book_detail, build_book_list_items};
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
    let detail = BookRepository::new(&state.repo).get_detail(id).await?;
    let book = build_book_detail(
        detail.id,
        detail.name,
        detail.original_title,
        detail.publisher,
        detail.format,
        detail.series,
        detail.series_index,
        detail.publication_date,
        detail.isbn,
        detail.notes,
        detail.has_cover,
        detail.has_paper,
        detail.contents,
        detail.people,
        detail.tags,
        detail.languages,
    );

    let mut ctx = Context::new();
    ctx.insert("book", &book);

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
