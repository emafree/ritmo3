use axum::extract::{Path, State};
use axum::response::Html;
use ritmo_presenter::build_person_list_items;
use ritmo_repository::PersonRepository;
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let rows = PersonRepository::list_all_for_display(state.repo.pool()).await?;
    let people = build_person_list_items(rows);

    let mut ctx = Context::new();
    ctx.insert("people", &people);

    let body = state
        .tera
        .render("people/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

pub async fn detail(Path(id): Path<i64>) -> Result<Html<String>, WebError> {
    let tpl = include_str!("../templates/people/detail.html");
    Ok(Html(tpl.replace("{{id}}", &id.to_string())))
}

pub async fn form() -> Result<Html<String>, WebError> {
    let body = include_str!("../templates/people/form.html");
    Ok(Html(body.to_owned()))
}
