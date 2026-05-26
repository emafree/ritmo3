use axum::extract::{Form, Path, State};
use axum::response::{Html, IntoResponse, Redirect};
use ritmo_core::CoreContext;
use ritmo_domain::{Content, PartialDate};
use ritmo_presenter::{
    build_content_detail, build_content_list_items, ContentDetail, ContentFormData, LookupItem,
};
use ritmo_repository::{ContentDetailData, ContentRepository};
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let rows = ContentRepository::list_all_with_people(state.repo.pool()).await?;
    let contents = build_content_list_items(rows);

    let mut ctx = Context::new();
    ctx.insert("contents", &contents);

    let body = state
        .tera
        .render("contents/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, WebError> {
    let detail = ContentRepository::new(&state.repo).get_detail(id).await?;
    let form = build_content_form_data(&detail);
    let content = build_content_detail_vm(detail);
    render_page(&state, Some(content), form, false, None).await
}

pub async fn form(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    render_page(&state, None, ContentFormData::default(), true, None).await
}

pub async fn save(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<ContentFormData>,
) -> Result<impl IntoResponse, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let content = to_domain_content(id, &form);

    match ritmo_core::content::update(&core, &content).await {
        Ok(()) => Ok(Redirect::to(&format!("/contents/{id}")).into_response()),
        Err(err) => {
            let detail = ContentRepository::new(&state.repo)
                .get_detail(id)
                .await
                .ok();
            let content = detail.map(build_content_detail_vm);
            let page = render_page(&state, content, form, false, Some(err.to_string())).await?;
            Ok(page.into_response())
        }
    }
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<ContentFormData>,
) -> Result<impl IntoResponse, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let content = to_domain_content(0, &form);

    match ritmo_core::content::create(&core, &content).await {
        Ok(id) => Ok(Redirect::to(&format!("/contents/{id}")).into_response()),
        Err(err) => {
            let page = render_page(&state, None, form, true, Some(err.to_string())).await?;
            Ok(page.into_response())
        }
    }
}

async fn render_page(
    state: &AppState,
    content: Option<ContentDetail>,
    form: ContentFormData,
    is_new: bool,
    error: Option<String>,
) -> Result<Html<String>, WebError> {
    let content_types = load_content_types(state).await?;

    let mut ctx = Context::new();
    ctx.insert("content", &content);
    ctx.insert("form", &form);
    ctx.insert("types", &content_types);
    ctx.insert("is_new", &is_new);
    ctx.insert("error", &error);

    let body = state
        .tera
        .render("contents/detail.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

async fn load_content_types(state: &AppState) -> Result<Vec<LookupItem>, WebError> {
    let core = CoreContext::new(state.repo.clone());

    Ok(ritmo_core::content_type::list_all_with_label(&core, "it")
        .await?
        .into_iter()
        .map(|(id, _key, label)| LookupItem { id, label })
        .collect())
}

fn build_content_detail_vm(detail: ContentDetailData) -> ContentDetail {
    build_content_detail(
        detail.id,
        detail.name,
        detail.original_title,
        detail.content_type,
        detail.publication_date,
        detail.notes,
        detail.books,
        detail.people,
        detail.tags,
        detail.languages,
    )
}

fn build_content_form_data(detail: &ContentDetailData) -> ContentFormData {
    ContentFormData {
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
        notes: detail.notes.clone(),
        type_id: detail.type_id,
    }
}

fn to_domain_content(id: i64, form: &ContentFormData) -> Content {
    Content {
        id,
        title: form.name.trim().to_owned(),
        original_title: normalize_optional(&form.original_title),
        type_id: form.type_id,
        publication_year: build_partial_date(
            form.publication_date_year,
            form.publication_date_month,
            form.publication_date_day,
            form.publication_date_circa,
        ),
        notes: normalize_optional(&form.notes),
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
