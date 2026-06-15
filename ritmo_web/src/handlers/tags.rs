use axum::extract::{Form, Path, Query, State};
use axum::response::Html;
use ritmo_core::{rel_book_tag, rel_content_tag, tag};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct TagSearchQuery {
    q: Option<String>,
    entity_type: String,
    entity_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct LinkTagForm {
    tag_id: Option<i64>,
    tag_name: Option<String>,
    tag_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct TagResult {
    id: i64,
    name: String,
    tag_type: String,
}

fn validate_entity_type(entity_type: &str) -> Result<(), ritmo_errors::RitmoErr> {
    match entity_type {
        "books" | "contents" => Ok(()),
        _ => Err(ritmo_errors::RitmoErr::InvalidInput(format!(
            "unknown entity_type: {entity_type}"
        ))),
    }
}

pub async fn search(
    State(state): State<AppState>,
    Query(query): Query<TagSearchQuery>,
) -> Result<Html<String>, WebError> {
    validate_entity_type(&query.entity_type)?;
    let q = query.q.clone().unwrap_or_default();
    let trimmed = q.trim().to_owned();
    let results: Vec<TagResult> = if trimmed.is_empty() {
        Vec::new()
    } else {
        tag::search(&state.core, &trimmed)
            .await?
            .into_iter()
            .map(|t| TagResult {
                id: t.id,
                name: t.name,
                tag_type: t.tag_type,
            })
            .collect()
    };

    let create_name = if !trimmed.is_empty() {
        Some(trimmed.clone())
    } else {
        None
    };

    let mut ctx = Context::new();
    ctx.insert("tag_entity_type", &query.entity_type);
    ctx.insert("tag_entity_id", &query.entity_id);
    ctx.insert("results", &results);
    ctx.insert("create_name", &create_name);
    let html = state
        .tera
        .render("widgets/tag_search_results.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn link(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, i64)>,
    Form(form): Form<LinkTagForm>,
) -> Result<Html<String>, WebError> {
    let resolved_tag = if let Some(tag_id) = form.tag_id {
        tag::get(&state.core, tag_id).await?
    } else if let Some(ref tag_name) = form.tag_name {
        let tag_type = form.tag_type.as_deref().unwrap_or("personal");
        tag::get_or_create(&state.core, tag_name, tag_type).await?
    } else {
        return Err(ritmo_errors::RitmoErr::InvalidInput("tag_id or tag_name required".to_owned()).into());
    };

    let tag_id = resolved_tag.id;

    match entity_type.as_str() {
        "books" => rel_book_tag::link(&state.core, entity_id, tag_id).await?,
        "contents" => rel_content_tag::link(&state.core, entity_id, tag_id).await?,
        _ => return Err(ritmo_errors::RitmoErr::InvalidInput(format!("unknown entity_type: {entity_type}")).into()),
    }

    let tag = ritmo_presenter::TagBadge {
        tag_id: resolved_tag.id,
        name: resolved_tag.name,
        tag_type: resolved_tag.tag_type,
    };

    let mut ctx = Context::new();
    ctx.insert("tag_entity_type", &entity_type);
    ctx.insert("tag_entity_id", &entity_id);
    ctx.insert("tag", &tag);
    let html = state
        .tera
        .render("widgets/tag_badge.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn unlink(
    State(state): State<AppState>,
    Path((entity_type, entity_id, tag_id)): Path<(String, i64, i64)>,
) -> Result<Html<String>, WebError> {
    match entity_type.as_str() {
        "books" => rel_book_tag::unlink(&state.core, entity_id, tag_id).await?,
        "contents" => rel_content_tag::unlink(&state.core, entity_id, tag_id).await?,
        _ => return Err(ritmo_errors::RitmoErr::InvalidInput(format!("unknown entity_type: {entity_type}")).into()),
    }
    Ok(Html(String::new()))
}
