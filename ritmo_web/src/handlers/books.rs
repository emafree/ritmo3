use axum::extract::{Path, State};
use axum::response::Html;

use crate::error::WebError;
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let _ = (&state.pool, &state.config.database_url);
    let body = include_str!("../templates/books/list.html");
    Ok(Html(body.to_owned()))
}

pub async fn detail(Path(id): Path<i64>) -> Result<Html<String>, WebError> {
    let tpl = include_str!("../templates/books/detail.html");
    Ok(Html(tpl.replace("{{id}}", &id.to_string())))
}

pub async fn form() -> Result<Html<String>, WebError> {
    let body = include_str!("../templates/books/form.html");
    Ok(Html(body.to_owned()))
}
