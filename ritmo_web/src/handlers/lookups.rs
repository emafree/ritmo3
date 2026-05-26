use axum::response::Html;

use crate::error::WebError;

pub async fn tags() -> Result<Html<String>, WebError> {
    Ok(Html("<h1>Tags</h1>".to_owned()))
}

pub async fn publishers() -> Result<Html<String>, WebError> {
    Ok(Html("<h1>Publishers</h1>".to_owned()))
}

pub async fn series() -> Result<Html<String>, WebError> {
    Ok(Html("<h1>Series</h1>".to_owned()))
}

pub async fn formats() -> Result<Html<String>, WebError> {
    Ok(Html("<h1>Formats</h1>".to_owned()))
}

pub async fn roles() -> Result<Html<String>, WebError> {
    Ok(Html("<h1>Roles</h1>".to_owned()))
}
