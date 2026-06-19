use axum::extract::{Form, Path, Query, State};
use axum::response::Html;
use ritmo_core::{book, content, content_type, format, lookup, publisher, series};
use serde::Deserialize;
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

const DEFAULT_LOCALE: &str = "it";

#[derive(Debug, Deserialize)]
pub struct LookupSearchQuery {
    q: Option<String>,
    entity_type: String,
    entity_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct SetLookupForm {
    lookup_id: Option<i64>,
    lookup_value: Option<String>,
}

pub async fn search(
    State(state): State<AppState>,
    Path(lookup_kind): Path<String>,
    Query(query): Query<LookupSearchQuery>,
) -> Result<Html<String>, WebError> {
    let entity_type = lookup::LookupEntityType::parse(&query.entity_type)?;
    let lookup_kind = lookup::LookupKind::parse(&lookup_kind)?;
    validate_combination(entity_type, lookup_kind)?;

    let q = query.q.unwrap_or_default();
    let trimmed = q.trim().to_owned();
    let results = if trimmed.is_empty() {
        Vec::new()
    } else {
        search_results(&state, lookup_kind, &trimmed).await?
    };

    let mut ctx = Context::new();
    ctx.insert("lookup_kind", lookup_kind_str(lookup_kind));
    ctx.insert("entity_type", &query.entity_type);
    ctx.insert("entity_id", &query.entity_id);
    ctx.insert("results", &results);
    ctx.insert("lookup_value", &trimmed);
    let html = state
        .tera
        .render("widgets/lookup_search_results.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn set(
    State(state): State<AppState>,
    Path((entity_type, entity_id, lookup_kind)): Path<(String, i64, String)>,
    Form(form): Form<SetLookupForm>,
) -> Result<Html<String>, WebError> {
    let entity_type_parsed = lookup::LookupEntityType::parse(&entity_type)?;
    let lookup_kind_parsed = lookup::LookupKind::parse(&lookup_kind)?;
    validate_combination(entity_type_parsed, lookup_kind_parsed)?;

    if let Some(lookup_id) = form.lookup_id {
        lookup::set_lookup_by_id(&state.core, &entity_type, entity_id, &lookup_kind, lookup_id)
            .await?;
    } else if let Some(lookup_value) = form.lookup_value {
        lookup::set_lookup_by_value(
            &state.core,
            &entity_type,
            entity_id,
            &lookup_kind,
            &lookup_value,
        )
        .await?;
    } else {
        return Err(ritmo_errors::RitmoErr::InvalidInput(
            "lookup_id or lookup_value required".to_owned(),
        )
        .into());
    }

    render_lookup_widget(&state, &entity_type, entity_id, lookup_kind_parsed).await
}

pub async fn clear(
    State(state): State<AppState>,
    Path((entity_type, entity_id, lookup_kind)): Path<(String, i64, String)>,
) -> Result<Html<String>, WebError> {
    let entity_type_parsed = lookup::LookupEntityType::parse(&entity_type)?;
    let lookup_kind_parsed = lookup::LookupKind::parse(&lookup_kind)?;
    validate_combination(entity_type_parsed, lookup_kind_parsed)?;

    lookup::clear_lookup(&state.core, &entity_type, entity_id, &lookup_kind).await?;
    render_lookup_widget(&state, &entity_type, entity_id, lookup_kind_parsed).await
}

pub async fn render_lookup_widget(
    state: &AppState,
    entity_type: &str,
    entity_id: i64,
    lookup_kind: lookup::LookupKind,
) -> Result<Html<String>, WebError> {
    let widget_state = load_widget_state(state, entity_type, entity_id, lookup_kind).await?;
    let mut ctx = Context::new();
    ctx.insert("lookup_kind", lookup_kind_str(lookup_kind));
    ctx.insert("entity_type", entity_type);
    ctx.insert("entity_id", &entity_id);
    ctx.insert("lookup_state", &widget_state);
    ctx.insert("lookup_label", &lookup_label(lookup_kind));
    ctx.insert("lookup_placeholder", &lookup_placeholder(lookup_kind));
    let html = state
        .tera
        .render("widgets/lookup_select.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

async fn search_results(
    state: &AppState,
    lookup_kind: lookup::LookupKind,
    query: &str,
) -> Result<Vec<ritmo_presenter::LookupSearchResultItem>, WebError> {
    Ok(match lookup_kind {
        lookup::LookupKind::Publisher => {
            ritmo_presenter::build_publisher_lookup_items(publisher::search(&state.core, query).await?)
        }
        lookup::LookupKind::Series => {
            ritmo_presenter::build_series_lookup_items(series::search(&state.core, query).await?)
        }
        lookup::LookupKind::Format => {
            ritmo_presenter::build_format_lookup_items(
                format::search(&state.core, query).await?,
                DEFAULT_LOCALE,
            )
        }
        lookup::LookupKind::Type => ritmo_presenter::build_content_type_lookup_items(
            content_type::search(&state.core, query).await?,
            DEFAULT_LOCALE,
        ),
    })
}

async fn load_widget_state(
    state: &AppState,
    entity_type: &str,
    entity_id: i64,
    lookup_kind: lookup::LookupKind,
) -> Result<ritmo_presenter::LookupWidgetState, WebError> {
    let current = match lookup_kind {
        lookup::LookupKind::Publisher => {
            let book = book::get(&state.core, entity_id).await?;
            match book.publisher_id {
                Some(lookup_id) => Some(ritmo_presenter::build_publisher_lookup_item(
                    publisher::get(&state.core, lookup_id).await?,
                )),
                None => None,
            }
        }
        lookup::LookupKind::Series => {
            let book = book::get(&state.core, entity_id).await?;
            match book.series_id {
                Some(lookup_id) => Some(ritmo_presenter::build_series_lookup_item(
                    series::get(&state.core, lookup_id).await?,
                )),
                None => None,
            }
        }
        lookup::LookupKind::Format => {
            let book = book::get(&state.core, entity_id).await?;
            match book.format_id {
                Some(lookup_id) => Some(ritmo_presenter::build_format_lookup_item(
                    format::get(&state.core, lookup_id).await?,
                    DEFAULT_LOCALE,
                )),
                None => None,
            }
        }
        lookup::LookupKind::Type => {
            let content = content::get(&state.core, entity_id).await?;
            match content.type_id {
                Some(lookup_id) => Some(ritmo_presenter::build_content_type_lookup_item(
                    content_type::get(&state.core, lookup_id).await?,
                    DEFAULT_LOCALE,
                )),
                None => None,
            }
        }
    };

    if entity_type == "books" || entity_type == "contents" {
        Ok(ritmo_presenter::build_lookup_widget_state(current))
    } else {
        Err(ritmo_errors::RitmoErr::InvalidInput(format!(
            "unknown entity_type: {entity_type}"
        ))
        .into())
    }
}

fn validate_combination(
    entity_type: lookup::LookupEntityType,
    lookup_kind: lookup::LookupKind,
) -> Result<(), ritmo_errors::RitmoErr> {
    match (entity_type, lookup_kind) {
        (
            lookup::LookupEntityType::Books,
            lookup::LookupKind::Publisher | lookup::LookupKind::Series | lookup::LookupKind::Format,
        )
        | (lookup::LookupEntityType::Contents, lookup::LookupKind::Type) => Ok(()),
        _ => Err(ritmo_errors::RitmoErr::InvalidInput(
            "invalid entity_type / lookup_kind combination".to_owned(),
        )),
    }
}

fn lookup_kind_str(lookup_kind: lookup::LookupKind) -> &'static str {
    match lookup_kind {
        lookup::LookupKind::Publisher => "publisher",
        lookup::LookupKind::Series => "series",
        lookup::LookupKind::Format => "format",
        lookup::LookupKind::Type => "type",
    }
}

fn lookup_label(lookup_kind: lookup::LookupKind) -> &'static str {
    match lookup_kind {
        lookup::LookupKind::Publisher => "Editore",
        lookup::LookupKind::Series => "Collana",
        lookup::LookupKind::Format => "Formato",
        lookup::LookupKind::Type => "Tipo",
    }
}

fn lookup_placeholder(lookup_kind: lookup::LookupKind) -> &'static str {
    match lookup_kind {
        lookup::LookupKind::Publisher => "Cerca o crea editore",
        lookup::LookupKind::Series => "Cerca o crea collana",
        lookup::LookupKind::Format => "Cerca o crea formato",
        lookup::LookupKind::Type => "Cerca o crea tipo",
    }
}

#[cfg(test)]
mod tests {
    use super::{render_lookup_widget, search, validate_combination, LookupSearchQuery};
    use crate::state::{load_tera, AppConfig, AppState};
    use axum::extract::{Path, Query, State};
    use ritmo_core::CoreContext;
    use ritmo_domain::{Book, Content};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    fn app_state(core: CoreContext) -> AppState {
        AppState::new(
            core,
            AppConfig {
                bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 3001),
                database_url: "sqlite::memory:".to_owned(),
            },
            load_tera().unwrap(),
        )
    }

    #[test]
    fn validate_combination_rejects_invalid_pairs() {
        assert!(validate_combination(
            lookup::LookupEntityType::Books,
            lookup::LookupKind::Publisher
        )
        .is_ok());
        assert!(validate_combination(
            lookup::LookupEntityType::Contents,
            lookup::LookupKind::Type
        )
        .is_ok());
        assert!(validate_combination(
            lookup::LookupEntityType::Books,
            lookup::LookupKind::Type
        )
        .is_err());
    }

    #[tokio::test]
    async fn search_renders_create_row_for_typed_query() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let state = app_state(core);

        let html = search(
            State(state),
            Path("publisher".to_owned()),
            Query(LookupSearchQuery {
                q: Some("Nuovo Editore".to_owned()),
                entity_type: "books".to_owned(),
                entity_id: 1,
            }),
        )
        .await
        .unwrap()
        .0;

        assert!(html.contains("Aggiungi"));
        assert!(html.contains("Nuovo Editore"));
    }

    #[tokio::test]
    async fn render_lookup_widget_shows_remove_button_only_when_value_exists() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let state = app_state(core.clone());
        let book_id = ritmo_core::book::create(
            &core,
            &Book {
                id: 0,
                title: "Libro".to_owned(),
                original_title: None,
                publisher_id: None,
                format_id: None,
                series_id: None,
                series_index: None,
                isbn: None,
                publication_year: None,
                notes: None,
                has_cover: false,
                has_paper: false,
            },
        )
        .await
        .unwrap();
        let content_id = ritmo_core::content::create(
            &core,
            &Content {
                id: 0,
                title: "Contenuto".to_owned(),
                original_title: None,
                type_id: None,
                publication_year: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let empty_html = render_lookup_widget(&state, "books", book_id, lookup::LookupKind::Publisher)
            .await
            .unwrap()
            .0;
        assert!(!empty_html.contains("Rimuovi"));

        ritmo_core::lookup::set_lookup_by_value(&core, "contents", content_id, "type", "inline_type")
            .await
            .unwrap();
        let filled_html = render_lookup_widget(&state, "contents", content_id, lookup::LookupKind::Type)
            .await
            .unwrap()
            .0;
        assert!(filled_html.contains("inline_type"));
        assert!(filled_html.contains("Rimuovi"));
    }
}
