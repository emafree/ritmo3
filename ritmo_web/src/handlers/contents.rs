use axum::extract::{Path, State};
use axum::response::Html;
use ritmo_presenter::{build_content_detail, build_content_list_items};
use ritmo_repository::ContentRepository;
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let rows = ContentRepository::list_all_with_people(state.repo.pool()).await?;
    let contents = build_content_list_items(rows);

    let mut ctx = Context::new();
    ctx.insert("contents", &contents);

    let body = state
        .tera
        .render("contents/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, WebError> {
    let detail = ContentRepository::new(&state.repo).get_detail(id).await?;
    let content = build_content_detail(
        detail.id,
        detail.name,
        detail.original_title,
        detail.content_type,
        detail.publication_date,
        detail.notes,
        detail.books,
        detail.people,
        detail.tags,
        detail.languages,
    );

    let mut ctx = Context::new();
    ctx.insert("content", &content);

    let body = state
        .tera
        .render("contents/detail.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

pub async fn form() -> Result<Html<String>, WebError> {
    let body = include_str!("../templates/contents/form.html");
    Ok(Html(body.to_owned()))
}
