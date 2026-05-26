use axum::extract::{Path, State};
use axum::response::Html;
use ritmo_presenter::build_content_list_items;
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

pub async fn detail(Path(id): Path<i64>) -> Result<Html<String>, WebError> {
    let tpl = include_str!("../templates/contents/detail.html");
    Ok(Html(tpl.replace("{{id}}", &id.to_string())))
}

pub async fn form() -> Result<Html<String>, WebError> {
    let body = include_str!("../templates/contents/form.html");
    Ok(Html(body.to_owned()))
}
