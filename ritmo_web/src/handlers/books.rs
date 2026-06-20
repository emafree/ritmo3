use axum::extract::{Form, Path, State};
use axum::response::Html;
use ritmo_core::book;
use ritmo_core::lookup;
use ritmo_domain::{Book, PartialDate};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::error::WebError;
use crate::handlers::lookups::render_lookup_widget;
use crate::state::AppState;

#[derive(Debug, Deserialize, Clone)]
pub struct BookFormData {
    title: String,
    original_title: Option<String>,
    isbn: Option<String>,
    notes: Option<String>,
    series_index: Option<String>,
    publication_date_year: Option<String>,
    publication_date_month: Option<String>,
    publication_date_day: Option<String>,
    #[serde(default)]
    publication_date_circa: bool,
    #[serde(default)]
    has_cover: bool,
    #[serde(default)]
    has_paper: bool,
}

#[derive(Debug, Clone, Serialize)]
struct BookFormView {
    id: Option<i64>,
    title: String,
    original_title: Option<String>,
    isbn: Option<String>,
    notes: Option<String>,
    series_index: Option<i64>,
    publication_date_year: Option<i32>,
    publication_date_month: Option<u8>,
    publication_date_day: Option<u8>,
    publication_date_circa: bool,
    has_cover: bool,
    has_paper: bool,
}

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let mut ctx = Context::new();
    ctx.insert("current_section", "books");
    ctx.insert("books", &load_list_items(&state).await?);
    let html = state
        .tera
        .render("books/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(book_id): Path<i64>,
) -> Result<Html<String>, WebError> {
    render_popup_for_id(&state, book_id, None).await
}

pub async fn new_form(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    render_popup_from_form(&state, None, empty_form_view(), None).await
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<BookFormData>,
) -> Result<Html<String>, WebError> {
    let parsed = parse_form_into_book(0, &form, None, None, None)?;
    match book::create(&state.core, &parsed).await {
        Ok(book_id) => {
            let popup_html = render_popup_for_id(&state, book_id, None).await?.0;
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
    Path(book_id): Path<i64>,
    Form(form): Form<BookFormData>,
) -> Result<Html<String>, WebError> {
    let current = book::get(&state.core, book_id).await?;
    let parsed = parse_form_into_book(
        book_id,
        &form,
        current.publisher_id,
        current.format_id,
        current.series_id,
    )?;
    match book::update(&state.core, &parsed).await {
        Ok(()) => Ok(Html(format!(
            "{}{}",
            list_update_oob(&state).await?,
            popup_root_oob("")
        ))),
        Err(err) => {
            render_popup_from_form(
                &state,
                Some(book_id),
                form_into_view(Some(book_id), &form)?,
                Some(err.to_string()),
            )
            .await
        }
    }
}

async fn render_popup_for_id(
    state: &AppState,
    book_id: i64,
    error: Option<String>,
) -> Result<Html<String>, WebError> {
    let book = book::get(&state.core, book_id).await?;
    render_popup_from_form(state, Some(book_id), view_from_book(&book), error).await
}

async fn render_popup_from_form(
    state: &AppState,
    book_id: Option<i64>,
    form: BookFormView,
    error: Option<String>,
) -> Result<Html<String>, WebError> {
    let mut ctx = Context::new();
    let is_new = book_id.is_none();
    ctx.insert("is_new", &is_new);
    ctx.insert("form", &form);
    ctx.insert("save_url", if is_new { "/books" } else { "" });
    if let Some(id) = book_id {
        ctx.insert("save_url", &format!("/books/{id}"));
        let people = book::list_people_with_roles(&state.core, id).await?;
        let people_items = ritmo_presenter::build_people_role_items(
            people
                .into_iter()
                .map(|(person, role)| (person.id, person.name, role.id, role.i18n_key))
                .collect(),
        );
        ctx.insert("pr_entity_type", "books");
        ctx.insert("pr_entity_id", &id);
        ctx.insert("pr_people_roles", &people_items);

        let tags = book::list_tags(&state.core, id).await?;
        let tag_items = ritmo_presenter::build_tag_badges(
            tags.into_iter()
                .map(|t| (t.id, t.name, t.tag_type))
                .collect(),
        );
        ctx.insert("tag_entity_type", "books");
        ctx.insert("tag_entity_id", &id);
        ctx.insert("tag_items", &tag_items);

        ctx.insert(
            "publisher_lookup_widget",
            &render_lookup_widget(state, "books", id, lookup::LookupKind::Publisher)
                .await?
                .0,
        );
        ctx.insert(
            "series_lookup_widget",
            &render_lookup_widget(state, "books", id, lookup::LookupKind::Series)
                .await?
                .0,
        );
        ctx.insert(
            "format_lookup_widget",
            &render_lookup_widget(state, "books", id, lookup::LookupKind::Format)
                .await?
                .0,
        );
    }
    ctx.insert("widgets_enabled", &book_id.is_some());
    ctx.insert("error", &error);
    let html = state
        .tera
        .render("books/popup.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

async fn list_update_oob(state: &AppState) -> Result<String, WebError> {
    let rows_html = render_rows_html(state).await?;
    Ok(format!(
        r#"<div id="books-list" hx-swap-oob="innerHTML">{rows_html}</div>"#
    ))
}

fn popup_root_oob(content: &str) -> String {
    format!(r#"<div id="popup-root" hx-swap-oob="innerHTML">{content}</div>"#)
}

async fn render_rows_html(state: &AppState) -> Result<String, WebError> {
    let mut ctx = Context::new();
    ctx.insert("books", &load_list_items(state).await?);
    state
        .tera
        .render("books/list_rows.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")).into())
}

async fn load_list_items(state: &AppState) -> Result<Vec<ritmo_presenter::BookListItem>, WebError> {
    Ok(ritmo_presenter::build_book_list_items(
        book::list_all_for_display(&state.core).await?,
    ))
}

fn parse_form_into_book(
    id: i64,
    form: &BookFormData,
    publisher_id: Option<i64>,
    format_id: Option<i64>,
    series_id: Option<i64>,
) -> Result<Book, WebError> {
    Ok(Book {
        id,
        title: form.title.trim().to_owned(),
        original_title: trim_to_option(form.original_title.clone()),
        publisher_id,
        format_id,
        series_id,
        series_index: parse_optional_i64(form.series_index.clone())?,
        isbn: trim_to_option(form.isbn.clone()),
        publication_year: parse_partial_date(
            form.publication_date_year.clone(),
            form.publication_date_month.clone(),
            form.publication_date_day.clone(),
            form.publication_date_circa,
        )?,
        notes: trim_to_option(form.notes.clone()),
        has_cover: form.has_cover,
        has_paper: form.has_paper,
    })
}

fn empty_form_view() -> BookFormView {
    BookFormView {
        id: None,
        title: String::new(),
        original_title: None,
        isbn: None,
        notes: None,
        series_index: None,
        publication_date_year: None,
        publication_date_month: None,
        publication_date_day: None,
        publication_date_circa: false,
        has_cover: false,
        has_paper: false,
    }
}

fn view_from_book(book: &Book) -> BookFormView {
    BookFormView {
        id: Some(book.id),
        title: book.title.clone(),
        original_title: book.original_title.clone(),
        isbn: book.isbn.clone(),
        notes: book.notes.clone(),
        series_index: book.series_index,
        publication_date_year: book.publication_year.as_ref().and_then(|d| d.year),
        publication_date_month: book.publication_year.as_ref().and_then(|d| d.month),
        publication_date_day: book.publication_year.as_ref().and_then(|d| d.day),
        publication_date_circa: book.publication_year.as_ref().is_some_and(|d| d.circa),
        has_cover: book.has_cover,
        has_paper: book.has_paper,
    }
}

fn form_into_view(id: Option<i64>, form: &BookFormData) -> Result<BookFormView, WebError> {
    let partial_date = parse_partial_date(
        form.publication_date_year.clone(),
        form.publication_date_month.clone(),
        form.publication_date_day.clone(),
        form.publication_date_circa,
    )?;
    Ok(BookFormView {
        id,
        title: form.title.clone(),
        original_title: trim_to_option(form.original_title.clone()),
        isbn: trim_to_option(form.isbn.clone()),
        notes: trim_to_option(form.notes.clone()),
        series_index: parse_optional_i64(form.series_index.clone())?,
        publication_date_year: partial_date.as_ref().and_then(|d| d.year),
        publication_date_month: partial_date.as_ref().and_then(|d| d.month),
        publication_date_day: partial_date.as_ref().and_then(|d| d.day),
        publication_date_circa: partial_date.as_ref().is_some_and(|d| d.circa),
        has_cover: form.has_cover,
        has_paper: form.has_paper,
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

fn parse_optional_i64(value: Option<String>) -> Result<Option<i64>, WebError> {
    parse_optional(value, "numero intero")
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
    use super::{create, list, new_form, BookFormData};
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
    async fn list_marks_books_active_and_wires_global_new() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let html = list(State(app_state(core))).await.unwrap().0;
        assert!(html.contains(r#"href="/books""#));
        assert!(html.contains(r#"class="view-toggle-link active""#));
        assert!(html.contains(r#"hx-get="/books/new""#));
    }

    #[tokio::test]
    async fn create_keeps_popup_open_and_enables_widgets() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let state = app_state(core);
        let html = create(
            State(state),
            Form(BookFormData {
                title: "Libro popup".to_owned(),
                original_title: None,
                isbn: None,
                notes: None,
                series_index: None,
                publication_date_year: None,
                publication_date_month: None,
                publication_date_day: None,
                publication_date_circa: false,
                has_cover: false,
                has_paper: false,
            }),
        )
        .await
        .unwrap()
        .0;
        assert!(html.contains("hx-swap-oob"));
        assert!(html.contains("lookup-widget-books-1-publisher"));
    }
}
