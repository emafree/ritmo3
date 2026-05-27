use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use axum::Json;
use ritmo_core::CoreContext;
use serde::Serialize;
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct LookupListItem {
    pub id: i64,
    pub label: String,
    pub delete_path: String,
}

pub async fn tags(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let items = ritmo_core::tag::list_all(&core)
        .await?
        .into_iter()
        .map(|item| LookupListItem {
            id: item.id,
            label: format!("{} ({})", item.name, item.tag_type),
            delete_path: format!("/tags/{}", item.id),
        })
        .collect();
    render_lookup_page(&state, "Tag", "Nessun tag nel database.", items).await
}

pub async fn publishers(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let items = ritmo_core::publisher::list_all(&core)
        .await?
        .into_iter()
        .map(|item| LookupListItem {
            id: item.id,
            label: item.name,
            delete_path: format!("/publishers/{}", item.id),
        })
        .collect();
    render_lookup_page(&state, "Editori", "Nessun editore nel database.", items).await
}

pub async fn series(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let items = ritmo_core::series::list_all(&core)
        .await?
        .into_iter()
        .map(|item| LookupListItem {
            id: item.id,
            label: item.name,
            delete_path: format!("/series/{}", item.id),
        })
        .collect();
    render_lookup_page(&state, "Serie", "Nessuna serie nel database.", items).await
}

pub async fn formats(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let items = ritmo_core::format::list_all_with_label(&core, "it")
        .await?
        .into_iter()
        .map(|(id, key, label)| LookupListItem {
            id,
            label: if label == key {
                label
            } else {
                format!("{label} ({key})")
            },
            delete_path: format!("/formats/{id}"),
        })
        .collect();
    render_lookup_page(&state, "Formati", "Nessun formato nel database.", items).await
}

pub async fn types(State(state): State<AppState>) -> Result<Json<Vec<(i64, String, String)>>, WebError> {
    let core = CoreContext::new(state.repo);
    let items = ritmo_core::content_type::list_all_with_label(&core, "it")
        .await?;
    Ok(Json(items))
}

pub async fn roles(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let items = ritmo_core::role::list_all(&core)
        .await?
        .into_iter()
        .map(|item| LookupListItem {
            id: item.id,
            label: item.i18n_key,
            delete_path: format!("/roles/{}", item.id),
        })
        .collect();
    render_lookup_page(&state, "Ruoli", "Nessun ruolo nel database.", items).await
}

pub async fn languages(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let items = ritmo_core::language::list_all(&core)
        .await?
        .into_iter()
        .map(|item| {
            let code = item
                .iso_639_2
                .clone()
                .or(item.iso_639_3.clone())
                .map(|code| format!(" ({code})"))
                .unwrap_or_default();
            LookupListItem {
                id: item.id,
                label: format!("{}{}", item.name, code),
                delete_path: format!("/languages/{}", item.id),
            }
        })
        .collect();
    render_lookup_page(&state, "Lingue", "Nessuna lingua nel database.", items).await
}

pub async fn delete_publisher(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, WebError> {
    let core = CoreContext::new(state.repo.clone());
    ritmo_core::publisher::delete(&core, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_series(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, WebError> {
    let core = CoreContext::new(state.repo.clone());
    ritmo_core::series::delete(&core, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_format(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, WebError> {
    let core = CoreContext::new(state.repo.clone());
    ritmo_core::format::delete(&core, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_role(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, WebError> {
    let core = CoreContext::new(state.repo.clone());
    ritmo_core::role::delete(&core, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_language(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, WebError> {
    let core = CoreContext::new(state.repo.clone());
    ritmo_core::language::delete(&core, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_tag(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, WebError> {
    let core = CoreContext::new(state.repo.clone());
    ritmo_core::tag::delete(&core, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn render_lookup_page(
    state: &AppState,
    title: &str,
    empty_message: &str,
    items: Vec<LookupListItem>,
) -> Result<Html<String>, WebError> {
    let mut ctx = Context::new();
    ctx.insert("title", &title);
    ctx.insert("empty_message", &empty_message);
    ctx.insert("items", &items);

    let body = state
        .tera
        .render("lookups/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}
