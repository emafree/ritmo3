use axum::extract::{Form, Path, Query, State};
use axum::response::Html;
use ritmo_core::{book, content, rel_book_person, rel_content_person, role};
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
pub struct PersonSearchQuery {
    q: Option<String>,
    entity_type: String,
    entity_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct LinkForm {
    person_id: i64,
    role_id: i64,
}

#[derive(Debug, Clone, Serialize)]
struct PersonResult {
    id: i64,
    name: String,
}

#[derive(Debug, Clone, Serialize)]
struct RoleOption {
    id: i64,
    i18n_key: String,
}

fn validate_entity_type(entity_type: &str) -> Result<(), ritmo_errors::RitmoErr> {
    match entity_type {
        "books" | "contents" => Ok(()),
        _ => Err(ritmo_errors::RitmoErr::InvalidInput(format!(
            "unknown entity_type: {entity_type}"
        ))),
    }
}

async fn load_roles(state: &AppState) -> Result<Vec<RoleOption>, WebError> {
    Ok(role::list_all(&state.core)
        .await?
        .into_iter()
        .map(|r| RoleOption {
            id: r.id,
            i18n_key: r.i18n_key,
        })
        .collect())
}

pub async fn search_panel(
    State(state): State<AppState>,
    Query(query): Query<PanelQuery>,
) -> Result<Html<String>, WebError> {
    validate_entity_type(&query.entity_type)?;
    let roles = load_roles(&state).await?;

    let mut ctx = Context::new();
    ctx.insert("pr_entity_type", &query.entity_type);
    ctx.insert("pr_entity_id", &query.entity_id);
    ctx.insert("results", &Vec::<PersonResult>::new());
    ctx.insert("roles", &roles);
    let html = state
        .tera
        .render("widgets/people_roles_search_panel.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn person_search(
    State(state): State<AppState>,
    Query(query): Query<PersonSearchQuery>,
) -> Result<Html<String>, WebError> {
    validate_entity_type(&query.entity_type)?;
    let q = query.q.unwrap_or_default();
    let results: Vec<PersonResult> = if q.trim().is_empty() {
        Vec::new()
    } else {
        ritmo_core::person::search(&state.core, &q)
            .await?
            .into_iter()
            .map(|p| PersonResult {
                id: p.id,
                name: p.name,
            })
            .collect()
    };
    let roles = load_roles(&state).await?;

    let mut ctx = Context::new();
    ctx.insert("pr_entity_type", &query.entity_type);
    ctx.insert("pr_entity_id", &query.entity_id);
    ctx.insert("results", &results);
    ctx.insert("roles", &roles);
    let html = state
        .tera
        .render("widgets/people_search_results.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn link(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, i64)>,
    Form(form): Form<LinkForm>,
) -> Result<Html<String>, WebError> {
    match entity_type.as_str() {
        "books" => rel_book_person::link(&state.core, entity_id, form.person_id, form.role_id).await?,
        "contents" => rel_content_person::link(&state.core, entity_id, form.person_id, form.role_id).await?,
        _ => return Err(ritmo_errors::RitmoErr::InvalidInput(format!("unknown entity_type: {entity_type}")).into()),
    }

    let pairs = match entity_type.as_str() {
        "books" => book::list_people_with_roles(&state.core, entity_id).await?,
        _ => content::list_people_with_roles(&state.core, entity_id).await?,
    };

    let found = pairs
        .into_iter()
        .find(|(person, role)| person.id == form.person_id && role.id == form.role_id);

    if let Some((person, role)) = found {
        let item = ritmo_presenter::PeopleRoleItem {
            person_id: person.id,
            person_name: person.name,
            role_id: role.id,
            role_key: role.i18n_key,
        };
        let mut ctx = Context::new();
        ctx.insert("pr_entity_type", &entity_type);
        ctx.insert("pr_entity_id", &entity_id);
        ctx.insert("item", &item);
        let html = state
            .tera
            .render("widgets/people_roles_row.html", &ctx)
            .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
        Ok(Html(html))
    } else {
        Ok(Html(String::new()))
    }
}

pub async fn unlink(
    State(state): State<AppState>,
    Path((entity_type, entity_id, person_id, role_id)): Path<(String, i64, i64, i64)>,
) -> Result<Html<String>, WebError> {
    match entity_type.as_str() {
        "books" => rel_book_person::unlink(&state.core, entity_id, person_id, role_id).await?,
        "contents" => rel_content_person::unlink(&state.core, entity_id, person_id, role_id).await?,
        _ => return Err(ritmo_errors::RitmoErr::InvalidInput(format!("unknown entity_type: {entity_type}")).into()),
    }
    Ok(Html(String::new()))
}
