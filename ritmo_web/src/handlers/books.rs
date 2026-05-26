use crate::error::WebError;
use crate::state::AppState;
use axum::extract::{Form, Path, State};
use axum::response::{Html, IntoResponse, Redirect};
use ritmo_core::CoreContext;
use ritmo_domain::{Book, PartialDate};
use ritmo_presenter::{
    build_book_detail, build_book_list_items, BookDetail, BookFormData, LookupItem,
};
use ritmo_repository::{BookDetailData, BookRepository};
use tera::Context;

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let repo = BookRepository::new(&state.repo);
    let rows = repo.list_all_with_authors().await?;
    let books = build_book_list_items(rows);

    let mut ctx = Context::new();
    ctx.insert("books", &books);

    let body = state
        .tera
        .render("books/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, WebError> {
    let detail = BookRepository::new(&state.repo).get_detail(id).await?;
    let form = build_book_form_data(&detail);
    let book = build_book_detail_vm(detail);
    render_page(&state, Some(book), form, false, None).await
}

pub async fn form(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    render_page(&state, None, BookFormData::default(), true, None).await
}

pub async fn save(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<BookFormData>,
) -> Result<impl IntoResponse, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let book = to_domain_book(id, &form);

    match ritmo_core::book::update(&core, &book).await {
        Ok(()) => Ok(Redirect::to(&format!("/books/{id}")).into_response()),
        Err(err) => {
            let detail = BookRepository::new(&state.repo).get_detail(id).await.ok();
            let book = detail.map(build_book_detail_vm);
            let page = render_page(&state, book, form, false, Some(err.to_string())).await?;
            Ok(page.into_response())
        }
    }
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<BookFormData>,
) -> Result<impl IntoResponse, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let book = to_domain_book(0, &form);

    match ritmo_core::book::create(&core, &book).await {
        Ok(id) => Ok(Redirect::to(&format!("/books/{id}")).into_response()),
        Err(err) => {
            let page = render_page(&state, None, form, true, Some(err.to_string())).await?;
            Ok(page.into_response())
        }
    }
}

async fn render_page(
    state: &AppState,
    book: Option<BookDetail>,
    form: BookFormData,
    is_new: bool,
    error: Option<String>,
) -> Result<Html<String>, WebError> {
    let (publishers, formats, series) = load_book_lookups(state).await?;

    let mut ctx = Context::new();
    ctx.insert("book", &book);
    ctx.insert("form", &form);
    ctx.insert("publishers", &publishers);
    ctx.insert("formats", &formats);
    ctx.insert("series", &series);
    ctx.insert("is_new", &is_new);
    ctx.insert("error", &error);

    let body = state
        .tera
        .render("books/detail.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

async fn load_book_lookups(
    state: &AppState,
) -> Result<(Vec<LookupItem>, Vec<LookupItem>, Vec<LookupItem>), WebError> {
    let core = CoreContext::new(state.repo.clone());

    let publishers = ritmo_core::publisher::list_all(&core)
        .await?
        .into_iter()
        .map(|item| LookupItem {
            id: item.id,
            label: item.name,
        })
        .collect();

    let formats = ritmo_core::format::list_all_with_label(&core, "it")
        .await?
        .into_iter()
        .map(|(id, _key, label)| LookupItem { id, label })
        .collect();

    let series = ritmo_core::series::list_all(&core)
        .await?
        .into_iter()
        .map(|item| LookupItem {
            id: item.id,
            label: item.name,
        })
        .collect();

    Ok((publishers, formats, series))
}

fn build_book_detail_vm(detail: BookDetailData) -> BookDetail {
    build_book_detail(
        detail.id,
        detail.name,
        detail.original_title,
        detail.publisher,
        detail.format,
        detail.series,
        detail.series_index,
        detail.publication_date,
        detail.isbn,
        detail.notes,
        detail.has_cover,
        detail.has_paper,
        detail.contents,
        detail.people,
        detail.tags,
        detail.languages,
    )
}

fn build_book_form_data(detail: &BookDetailData) -> BookFormData {
    BookFormData {
        name: detail.name.clone(),
        original_title: detail.original_title.clone(),
        publication_date_year: detail.publication_date.as_ref().and_then(|date| date.year),
        publication_date_month: detail.publication_date.as_ref().and_then(|date| date.month),
        publication_date_day: detail.publication_date.as_ref().and_then(|date| date.day),
        publication_date_circa: detail
            .publication_date
            .as_ref()
            .map(|date| date.circa)
            .unwrap_or(false),
        isbn: detail.isbn.clone(),
        notes: detail.notes.clone(),
        has_cover: detail.has_cover,
        has_paper: detail.has_paper,
        publisher_id: detail.publisher_id,
        format_id: detail.format_id,
        series_id: detail.series_id,
        series_index: detail.series_index,
    }
}

fn to_domain_book(id: i64, form: &BookFormData) -> Book {
    Book {
        id,
        title: form.name.trim().to_owned(),
        original_title: normalize_optional(&form.original_title),
        publisher_id: form.publisher_id,
        format_id: form.format_id,
        series_id: form.series_id,
        series_index: form.series_index,
        isbn: normalize_optional(&form.isbn),
        publication_year: build_partial_date(
            form.publication_date_year,
            form.publication_date_month,
            form.publication_date_day,
            form.publication_date_circa,
        ),
        notes: normalize_optional(&form.notes),
        has_cover: form.has_cover,
        has_paper: form.has_paper,
    }
}

fn build_partial_date(
    year: Option<i32>,
    month: Option<u8>,
    day: Option<u8>,
    circa: bool,
) -> Option<PartialDate> {
    if year.is_none() && month.is_none() && day.is_none() && !circa {
        return None;
    }

    Some(PartialDate {
        year,
        month,
        day,
        circa,
    })
}

fn normalize_optional(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .map(|text| text.trim().to_owned())
        .filter(|text| !text.is_empty())
}
