use axum::extract::{Form, Path, State};
use axum::response::Html;
use ritmo_core::content;
use ritmo_core::lookup;
use ritmo_domain::{Content, PartialDate};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::error::WebError;
use crate::handlers::lookups::render_lookup_widget;
use crate::state::AppState;

#[derive(Debug, Deserialize, Clone)]
pub struct ContentFormData {
    title: String,
    original_title: Option<String>,
    notes: Option<String>,
    publication_date_year: Option<String>,
    publication_date_month: Option<String>,
    publication_date_day: Option<String>,
    #[serde(default)]
    publication_date_circa: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ContentFormView {
    id: Option<i64>,
    title: String,
    original_title: Option<String>,
    notes: Option<String>,
    publication_date_year: Option<i32>,
    publication_date_month: Option<u8>,
    publication_date_day: Option<u8>,
    publication_date_circa: bool,
}

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let mut ctx = Context::new();
    ctx.insert("current_section", "contents");
    ctx.insert("contents", &load_list_items(&state).await?);
    let html = state
        .tera
        .render("contents/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(content_id): Path<i64>,
) -> Result<Html<String>, WebError> {
    render_popup_for_id(&state, content_id, None).await
}

pub async fn new_form(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    render_popup_from_form(&state, None, empty_form_view(), None).await
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<ContentFormData>,
) -> Result<Html<String>, WebError> {
    let parsed = parse_form_into_content(0, &form, None)?;
    match content::create(&state.core, &parsed).await {
        Ok(content_id) => {
            let popup_html = render_popup_for_id(&state, content_id, None).await?.0;
            Ok(Html(format!(
                "{}{}",
                list_update_oob(&state).await?,
                popup_root_oob(&popup_html)
            )))
        }
        Err(err) => {
            render_popup_from_form(
                &state,
                None,
                form_into_view(None, &form)?,
                Some(err.to_string()),
            )
            .await
        }
    }
}

pub async fn update(
    State(state): State<AppState>,
    Path(content_id): Path<i64>,
    Form(form): Form<ContentFormData>,
) -> Result<Html<String>, WebError> {
    let current = content::get(&state.core, content_id).await?;
    let parsed = parse_form_into_content(content_id, &form, current.type_id)?;
    match content::update(&state.core, &parsed).await {
        Ok(()) => Ok(Html(format!(
            "{}{}",
            list_update_oob(&state).await?,
            popup_root_oob("")
        ))),
        Err(err) => {
            render_popup_from_form(
                &state,
                Some(content_id),
                form_into_view(Some(content_id), &form)?,
                Some(err.to_string()),
            )
            .await
        }
    }
}

async fn render_popup_for_id(
    state: &AppState,
    content_id: i64,
    error: Option<String>,
) -> Result<Html<String>, WebError> {
    let content = content::get(&state.core, content_id).await?;
    render_popup_from_form(state, Some(content_id), view_from_content(&content), error).await
}

async fn render_popup_from_form(
    state: &AppState,
    content_id: Option<i64>,
    form: ContentFormView,
    error: Option<String>,
) -> Result<Html<String>, WebError> {
    let mut ctx = Context::new();
    let is_new = content_id.is_none();
    ctx.insert("is_new", &is_new);
    ctx.insert("form", &form);
    ctx.insert("save_url", if is_new { "/contents" } else { "" });
    if let Some(id) = content_id {
        ctx.insert("save_url", &format!("/contents/{id}"));

        let people = content::list_people_with_roles(&state.core, id).await?;
        let people_items = ritmo_presenter::build_people_role_items(
            people
                .into_iter()
                .map(|(person, role)| (person.id, person.name, role.id, role.i18n_key))
                .collect(),
        );
        ctx.insert("pr_entity_type", "contents");
        ctx.insert("pr_entity_id", &id);
        ctx.insert("pr_people_roles", &people_items);

        let tags = content::list_tags(&state.core, id).await?;
        let tag_items = ritmo_presenter::build_tag_badges(
            tags.into_iter()
                .map(|t| (t.id, t.name, t.tag_type))
                .collect(),
        );
        ctx.insert("tag_entity_type", "contents");
        ctx.insert("tag_entity_id", &id);
        ctx.insert("tag_items", &tag_items);

        let languages = content::list_languages_with_roles(&state.core, id).await?;
        ctx.insert("lang_entity_type", "contents");
        ctx.insert("lang_entity_id", &id);
        ctx.insert(
            "lang_items",
            &ritmo_presenter::build_lang_widget_items(languages),
        );

        ctx.insert(
            "type_lookup_widget",
            &render_lookup_widget(state, "contents", id, lookup::LookupKind::Type)
                .await?
                .0,
        );
    }
    ctx.insert("widgets_enabled", &content_id.is_some());
    ctx.insert("error", &error);
    let html = state
        .tera
        .render("contents/popup.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

async fn list_update_oob(state: &AppState) -> Result<String, WebError> {
    let rows_html = render_rows_html(state).await?;
    Ok(format!(
        r#"<div id="contents-list" hx-swap-oob="innerHTML">{rows_html}</div>"#
    ))
}

fn popup_root_oob(content: &str) -> String {
    format!(r#"<div id="popup-root" hx-swap-oob="innerHTML">{content}</div>"#)
}

async fn render_rows_html(state: &AppState) -> Result<String, WebError> {
    let mut ctx = Context::new();
    ctx.insert("contents", &load_list_items(state).await?);
    state
        .tera
        .render("contents/list_rows.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")).into())
}

async fn load_list_items(
    state: &AppState,
) -> Result<Vec<ritmo_presenter::ContentListItem>, WebError> {
    Ok(ritmo_presenter::build_content_list_items(
        content::list_all_for_display(&state.core).await?,
    ))
}

fn parse_form_into_content(
    id: i64,
    form: &ContentFormData,
    type_id: Option<i64>,
) -> Result<Content, WebError> {
    Ok(Content {
        id,
        title: form.title.trim().to_owned(),
        original_title: trim_to_option(form.original_title.clone()),
        type_id,
        publication_year: parse_partial_date(
            form.publication_date_year.clone(),
            form.publication_date_month.clone(),
            form.publication_date_day.clone(),
            form.publication_date_circa,
        )?,
        notes: trim_to_option(form.notes.clone()),
    })
}

fn empty_form_view() -> ContentFormView {
    ContentFormView {
        id: None,
        title: String::new(),
        original_title: None,
        notes: None,
        publication_date_year: None,
        publication_date_month: None,
        publication_date_day: None,
        publication_date_circa: false,
    }
}

fn view_from_content(content: &Content) -> ContentFormView {
    ContentFormView {
        id: Some(content.id),
        title: content.title.clone(),
        original_title: content.original_title.clone(),
        notes: content.notes.clone(),
        publication_date_year: content.publication_year.as_ref().and_then(|d| d.year),
        publication_date_month: content.publication_year.as_ref().and_then(|d| d.month),
        publication_date_day: content.publication_year.as_ref().and_then(|d| d.day),
        publication_date_circa: content.publication_year.as_ref().is_some_and(|d| d.circa),
    }
}

fn form_into_view(id: Option<i64>, form: &ContentFormData) -> Result<ContentFormView, WebError> {
    let partial_date = parse_partial_date(
        form.publication_date_year.clone(),
        form.publication_date_month.clone(),
        form.publication_date_day.clone(),
        form.publication_date_circa,
    )?;
    Ok(ContentFormView {
        id,
        title: form.title.clone(),
        original_title: trim_to_option(form.original_title.clone()),
        notes: trim_to_option(form.notes.clone()),
        publication_date_year: partial_date.as_ref().and_then(|d| d.year),
        publication_date_month: partial_date.as_ref().and_then(|d| d.month),
        publication_date_day: partial_date.as_ref().and_then(|d| d.day),
        publication_date_circa: partial_date.as_ref().is_some_and(|d| d.circa),
    })
}

fn parse_partial_date(
    year: Option<String>,
    month: Option<String>,
    day: Option<String>,
    circa: bool,
) -> Result<Option<PartialDate>, WebError> {
    let year = parse_optional_i32(year)?;
    let month = parse_optional_u8(month)?;
    let day = parse_optional_u8(day)?;
    if year.is_none() && month.is_none() && day.is_none() && !circa {
        Ok(None)
    } else {
        Ok(Some(PartialDate {
            year,
            month,
            day,
            circa,
        }))
    }
}

fn parse_optional_i32(value: Option<String>) -> Result<Option<i32>, WebError> {
    parse_optional(value, "anno")
}

fn parse_optional_u8(value: Option<String>) -> Result<Option<u8>, WebError> {
    parse_optional(value, "numero")
}

fn parse_optional<T: std::str::FromStr>(
    value: Option<String>,
    label: &str,
) -> Result<Option<T>, WebError> {
    match trim_to_option(value) {
        Some(text) => text.parse::<T>().map(Some).map_err(|_| {
            ritmo_errors::RitmoErr::InvalidInput(format!("Valore non valido per {label}: {text}"))
                .into()
        }),
        None => Ok(None),
    }
}

fn trim_to_option(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_owned())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{create, list, new_form, ContentFormData};
    use crate::state::{load_tera, AppConfig, AppState};
    use axum::extract::{Form, State};
    use ritmo_core::CoreContext;
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

    #[tokio::test]
    async fn new_popup_disables_relational_widgets() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let html = new_form(State(app_state(core))).await.unwrap().0;
        assert!(html.contains("overlay visible"));
        assert!(html.contains("I widget relazionali saranno disponibili"));
    }

    #[tokio::test]
    async fn list_marks_contents_active_and_wires_global_new() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let html = list(State(app_state(core))).await.unwrap().0;
        assert!(html.contains(r#"href="/contents""#));
        assert!(html.contains(r#"class="view-toggle-link active""#));
        assert!(html.contains(r#"hx-get="/contents/new""#));
    }

    #[tokio::test]
    async fn create_keeps_popup_open_and_enables_language_widget() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let state = app_state(core);
        let html = create(
            State(state),
            Form(ContentFormData {
                title: "Contenuto popup".to_owned(),
                original_title: None,
                notes: None,
                publication_date_year: None,
                publication_date_month: None,
                publication_date_day: None,
                publication_date_circa: false,
            }),
        )
        .await
        .unwrap()
        .0;
        assert!(html.contains("hx-swap-oob"));
        assert!(html.contains("lang-list-contents-1"));
    }
}
