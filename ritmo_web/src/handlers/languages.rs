use axum::extract::{Form, Path, Query, State};
use axum::response::Html;
use ritmo_core::{book, content, language, rel_book_language, rel_content_language};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct PanelQuery {
    entity_type: String,
    entity_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct LangSearchQuery {
    q: Option<String>,
    entity_type: String,
    entity_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct LinkForm {
    language_id: i64,
    role_id: i64,
}

#[derive(Debug, Clone, Serialize)]
struct LangResult {
    id: i64,
    name: String,
}

fn validate_entity_type(entity_type: &str) -> Result<(), ritmo_errors::RitmoErr> {
    match entity_type {
        "books" | "contents" => Ok(()),
        _ => Err(ritmo_errors::RitmoErr::InvalidInput(format!(
            "unknown entity_type: {entity_type}"
        ))),
    }
}

async fn load_roles(
    state: &AppState,
    entity_type: &str,
) -> Result<Vec<ritmo_presenter::LangRoleItem>, WebError> {
    let tuples = if entity_type == "books" {
        language::list_book_roles(&state.core).await?
    } else {
        language::list_content_roles(&state.core).await?
    };
    Ok(ritmo_presenter::build_lang_role_items(tuples))
}

pub async fn search_panel(
    State(state): State<AppState>,
    Query(query): Query<PanelQuery>,
) -> Result<Html<String>, WebError> {
    validate_entity_type(&query.entity_type)?;
    let lang_roles = load_roles(&state, &query.entity_type).await?;

    let mut ctx = Context::new();
    ctx.insert("lang_entity_type", &query.entity_type);
    ctx.insert("lang_entity_id", &query.entity_id);
    ctx.insert("results", &Vec::<LangResult>::new());
    ctx.insert("lang_roles", &lang_roles);
    let html = state
        .tera
        .render("widgets/lang_search_panel.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn lang_search(
    State(state): State<AppState>,
    Query(query): Query<LangSearchQuery>,
) -> Result<Html<String>, WebError> {
    validate_entity_type(&query.entity_type)?;
    let q = query.q.unwrap_or_default();
    let results: Vec<LangResult> = if q.trim().is_empty() {
        Vec::new()
    } else {
        language::search(&state.core, &q)
            .await?
            .into_iter()
            .map(|l| LangResult {
                id: l.id,
                name: l.name,
            })
            .collect()
    };
    let lang_roles = load_roles(&state, &query.entity_type).await?;

    let mut ctx = Context::new();
    ctx.insert("lang_entity_type", &query.entity_type);
    ctx.insert("lang_entity_id", &query.entity_id);
    ctx.insert("results", &results);
    ctx.insert("lang_roles", &lang_roles);
    let html = state
        .tera
        .render("widgets/lang_search_results.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn link(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, i64)>,
    Form(form): Form<LinkForm>,
) -> Result<Html<String>, WebError> {
    match entity_type.as_str() {
        "books" => rel_book_language::link(&state.core, entity_id, form.language_id, form.role_id).await?,
        "contents" => rel_content_language::link(&state.core, entity_id, form.language_id, form.role_id).await?,
        _ => return Err(ritmo_errors::RitmoErr::InvalidInput(format!("unknown entity_type: {entity_type}")).into()),
    }

    let pairs = match entity_type.as_str() {
        "books" => book::list_languages_with_roles(&state.core, entity_id).await?,
        _ => content::list_languages_with_roles(&state.core, entity_id).await?,
    };

    let found = pairs
        .into_iter()
        .find(|(lang_id, _, role_id, _)| *lang_id == form.language_id && *role_id == form.role_id);

    if let Some((language_id, official_name, role_id, role_name)) = found {
        let item = ritmo_presenter::LangWidgetItem {
            language_id,
            official_name,
            role_id,
            role_name,
        };
        let mut ctx = Context::new();
        ctx.insert("lang_entity_type", &entity_type);
        ctx.insert("lang_entity_id", &entity_id);
        ctx.insert("item", &item);
        let html = state
            .tera
            .render("widgets/lang_row.html", &ctx)
            .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
        Ok(Html(html))
    } else {
        Ok(Html(String::new()))
    }
}

pub async fn unlink(
    State(state): State<AppState>,
    Path((entity_type, entity_id, language_id, role_id)): Path<(String, i64, i64, i64)>,
) -> Result<Html<String>, WebError> {
    match entity_type.as_str() {
        "books" => rel_book_language::unlink(&state.core, entity_id, language_id, role_id).await?,
        "contents" => rel_content_language::unlink(&state.core, entity_id, language_id, role_id).await?,
        _ => return Err(ritmo_errors::RitmoErr::InvalidInput(format!("unknown entity_type: {entity_type}")).into()),
    }
    Ok(Html(String::new()))
}
