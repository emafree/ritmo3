use axum::extract::State;
use axum::response::Html;
use axum::Json;
use ritmo_core::CoreContext;
use serde::Serialize;

use crate::error::WebError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct NamedLookupItem {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct KeyLookupItem {
    pub id: i64,
    pub key: String,
    pub label: String,
}

pub async fn tags() -> Result<Html<String>, WebError> {
    Ok(Html("<h1>Tags</h1>".to_owned()))
}

pub async fn publishers(
    State(state): State<AppState>,
) -> Result<Json<Vec<NamedLookupItem>>, WebError> {
    let core = CoreContext::new(state.repo);
    let items = ritmo_core::publisher::list_all(&core)
        .await?
        .into_iter()
        .map(|item| NamedLookupItem {
            id: item.id,
            name: item.name,
        })
        .collect();
    Ok(Json(items))
}

pub async fn series(State(state): State<AppState>) -> Result<Json<Vec<NamedLookupItem>>, WebError> {
    let core = CoreContext::new(state.repo);
    let items = ritmo_core::series::list_all(&core)
        .await?
        .into_iter()
        .map(|item| NamedLookupItem {
            id: item.id,
            name: item.name,
        })
        .collect();
    Ok(Json(items))
}

pub async fn formats(State(state): State<AppState>) -> Result<Json<Vec<KeyLookupItem>>, WebError> {
    let core = CoreContext::new(state.repo);
    let items = ritmo_core::format::list_all_with_label(&core, "it")
        .await?
        .into_iter()
        .map(|(id, key, label)| KeyLookupItem { id, key, label })
        .collect();
    Ok(Json(items))
}

pub async fn types(State(state): State<AppState>) -> Result<Json<Vec<KeyLookupItem>>, WebError> {
    let core = CoreContext::new(state.repo);
    let items = ritmo_core::content_type::list_all_with_label(&core, "it")
        .await?
        .into_iter()
        .map(|(id, key, label)| KeyLookupItem { id, key, label })
        .collect();
    Ok(Json(items))
}

pub async fn roles() -> Result<Html<String>, WebError> {
    Ok(Html("<h1>Roles</h1>".to_owned()))
}
