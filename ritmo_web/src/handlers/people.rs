use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use ritmo_core::CoreContext;
use ritmo_domain::{PartialDate, Person};
use ritmo_presenter::{build_person_detail, build_person_list_items, PersonDetail, PersonFormData};
use ritmo_repository::{PersonDetailData, PersonRepository};
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let rows = PersonRepository::list_all_for_display(state.repo.pool()).await?;
    let people = build_person_list_items(rows);

    let mut ctx = Context::new();
    ctx.insert("people", &people);

    let body = state
        .tera
        .render("people/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, WebError> {
    let detail = PersonRepository::new(&state.repo).get_detail(id).await?;
    let form = build_person_form_data(&detail);
    let person = build_person_detail_vm(detail);
    render_page(&state, Some(person), form, false, None).await
}

pub async fn form(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    render_page(&state, None, PersonFormData::default(), true, None).await
}

pub async fn save(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<PersonFormData>,
) -> Result<impl IntoResponse, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let person = to_domain_person(id, &form);

    match ritmo_core::person::update(&core, &person).await {
        Ok(()) => Ok(Redirect::to(&format!("/people/{id}")).into_response()),
        Err(err) => {
            let detail = PersonRepository::new(&state.repo).get_detail(id).await.ok();
            let person = detail.map(build_person_detail_vm);
            let page = render_page(&state, person, form, false, Some(err.to_string())).await?;
            Ok(page.into_response())
        }
    }
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<PersonFormData>,
) -> Result<impl IntoResponse, WebError> {
    let core = CoreContext::new(state.repo.clone());
    let person = to_domain_person(0, &form);

    match ritmo_core::person::create(&core, &person).await {
        Ok(id) => Ok(Redirect::to(&format!("/people/{id}")).into_response()),
        Err(err) => {
            let page = render_page(&state, None, form, true, Some(err.to_string())).await?;
            Ok(page.into_response())
        }
    }
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, WebError> {
    let core = CoreContext::new(state.repo.clone());
    ritmo_core::person::delete(&core, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn render_page(
    state: &AppState,
    person: Option<PersonDetail>,
    form: PersonFormData,
    is_new: bool,
    error: Option<String>,
) -> Result<Html<String>, WebError> {
    let mut ctx = Context::new();
    ctx.insert("person", &person);
    ctx.insert("form", &form);
    ctx.insert("is_new", &is_new);
    ctx.insert("error", &error);

    let body = state
        .tera
        .render("people/detail.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

fn build_person_detail_vm(detail: PersonDetailData) -> PersonDetail {
    build_person_detail(
        detail.id,
        detail.name,
        detail.display_name,
        detail.birth_date,
        detail.death_date,
        detail.biography,
        detail.aliases,
        detail.places,
        detail.languages,
        detail.books,
        detail.contents,
    )
}

fn build_person_form_data(detail: &PersonDetailData) -> PersonFormData {
    PersonFormData {
        name: detail.name.clone(),
        display_name: detail.display_name.clone(),
        given_name: detail.given_name.clone(),
        surname: detail.surname.clone(),
        middle_names: detail.middle_names.clone(),
        title: detail.title.clone(),
        suffix: detail.suffix.clone(),
        birth_date_year: detail.birth_date.as_ref().and_then(|date| date.year),
        birth_date_month: detail.birth_date.as_ref().and_then(|date| date.month),
        birth_date_day: detail.birth_date.as_ref().and_then(|date| date.day),
        birth_date_circa: detail
            .birth_date
            .as_ref()
            .map(|date| date.circa)
            .unwrap_or(false),
        death_date_year: detail.death_date.as_ref().and_then(|date| date.year),
        death_date_month: detail.death_date.as_ref().and_then(|date| date.month),
        death_date_day: detail.death_date.as_ref().and_then(|date| date.day),
        death_date_circa: detail
            .death_date
            .as_ref()
            .map(|date| date.circa)
            .unwrap_or(false),
        biography: detail.biography.clone(),
        verified: detail.verified,
    }
}

fn to_domain_person(id: i64, form: &PersonFormData) -> Person {
    Person {
        id,
        name: form.name.trim().to_owned(),
        display_name: normalize_optional(&form.display_name),
        given_name: normalize_optional(&form.given_name),
        surname: normalize_optional(&form.surname),
        middle_names: normalize_optional(&form.middle_names),
        title: normalize_optional(&form.title),
        suffix: normalize_optional(&form.suffix),
        birth_date: build_partial_date(
            form.birth_date_year,
            form.birth_date_month,
            form.birth_date_day,
            form.birth_date_circa,
        ),
        death_date: build_partial_date(
            form.death_date_year,
            form.death_date_month,
            form.death_date_day,
            form.death_date_circa,
        ),
        biography: normalize_optional(&form.biography),
        verified: form.verified,
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
