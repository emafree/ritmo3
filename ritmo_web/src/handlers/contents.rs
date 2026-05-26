use axum::extract::Path;
use axum::response::Html;

use crate::error::WebError;

pub async fn list() -> Result<Html<String>, WebError> {
    let body = include_str!("../templates/contents/list.html");
    Ok(Html(body.to_owned()))
}

pub async fn detail(Path(id): Path<i64>) -> Result<Html<String>, WebError> {
    let tpl = include_str!("../templates/contents/detail.html");
    Ok(Html(tpl.replace("{{id}}", &id.to_string())))
}

pub async fn form() -> Result<Html<String>, WebError> {
    let body = include_str!("../templates/contents/form.html");
    Ok(Html(body.to_owned()))
}
