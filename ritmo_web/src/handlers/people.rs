use axum::extract::{Form, Path, State};
use axum::response::Html;
use ritmo_core::place::PlaceOwner;
use ritmo_core::{person, place};
use ritmo_domain::{PartialDate, Person};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

#[derive(Debug, Deserialize, Clone)]
pub struct PersonFormData {
    name: String,
    display_name: Option<String>,
    given_name: Option<String>,
    surname: Option<String>,
    middle_names: Option<String>,
    title: Option<String>,
    suffix: Option<String>,
    biography: Option<String>,
    birth_date_year: Option<String>,
    birth_date_month: Option<String>,
    birth_date_day: Option<String>,
    #[serde(default)]
    birth_date_circa: bool,
    death_date_year: Option<String>,
    death_date_month: Option<String>,
    death_date_day: Option<String>,
    #[serde(default)]
    death_date_circa: bool,
}

#[derive(Debug, Clone, Serialize)]
struct PersonFormView {
    id: Option<i64>,
    name: String,
    display_name: Option<String>,
    given_name: Option<String>,
    surname: Option<String>,
    middle_names: Option<String>,
    title: Option<String>,
    suffix: Option<String>,
    biography: Option<String>,
    birth_date_year: Option<i32>,
    birth_date_month: Option<u8>,
    birth_date_day: Option<u8>,
    birth_date_circa: bool,
    death_date_year: Option<i32>,
    death_date_month: Option<u8>,
    death_date_day: Option<u8>,
    death_date_circa: bool,
}

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let mut ctx = Context::new();
    ctx.insert("current_section", "people");
    ctx.insert("people", &load_list_items(&state).await?);
    let html = state
        .tera
        .render("people/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(person_id): Path<i64>,
) -> Result<Html<String>, WebError> {
    render_popup_for_id(&state, person_id, None).await
}

pub async fn new_form(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    render_popup_from_form(&state, None, empty_form_view(), None).await
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<PersonFormData>,
) -> Result<Html<String>, WebError> {
    let parsed = parse_form_into_person(0, &form)?;
    match person::create(&state.core, &parsed).await {
        Ok(person_id) => {
            let popup_html = render_popup_for_id(&state, person_id, None).await?.0;
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
    Path(person_id): Path<i64>,
    Form(form): Form<PersonFormData>,
) -> Result<Html<String>, WebError> {
    let parsed = parse_form_into_person(person_id, &form)?;
    match person::update(&state.core, &parsed).await {
        Ok(()) => Ok(Html(format!(
            "{}{}",
            list_update_oob(&state).await?,
            popup_root_oob("")
        ))),
        Err(err) => {
            render_popup_from_form(
                &state,
                Some(person_id),
                form_into_view(Some(person_id), &form)?,
                Some(err.to_string()),
            )
            .await
        }
    }
}

async fn render_popup_for_id(
    state: &AppState,
    person_id: i64,
    error: Option<String>,
) -> Result<Html<String>, WebError> {
    let person = person::get(&state.core, person_id).await?;
    render_popup_from_form(state, Some(person_id), view_from_person(&person), error).await
}

async fn render_popup_from_form(
    state: &AppState,
    person_id: Option<i64>,
    form: PersonFormView,
    error: Option<String>,
) -> Result<Html<String>, WebError> {
    let mut ctx = Context::new();
    let is_new = person_id.is_none();
    ctx.insert("is_new", &is_new);
    ctx.insert("form", &form);
    ctx.insert("save_url", if is_new { "/people" } else { "" });
    if let Some(id) = person_id {
        ctx.insert("save_url", &format!("/people/{id}"));
        let linked_places = place::list_linked(&state.core, PlaceOwner::Person(id)).await?;
        let places = linked_places
            .into_iter()
            .map(|row| {
                (
                    row.place_id,
                    row.continent,
                    row.country,
                    row.city,
                    row.circa,
                    row.disputed,
                    row.place_type_key,
                )
            })
            .collect();
        ctx.insert("entity_type", "people");
        ctx.insert("entity_id", &id);
        ctx.insert("places", &ritmo_presenter::build_place_items(places, "it"));
    }
    ctx.insert("widgets_enabled", &person_id.is_some());
    ctx.insert("error", &error);
    let html = state
        .tera
        .render("people/popup.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

async fn list_update_oob(state: &AppState) -> Result<String, WebError> {
    let rows_html = render_rows_html(state).await?;
    Ok(format!(
        r#"<div id="people-list" hx-swap-oob="innerHTML">{rows_html}</div>"#
    ))
}

fn popup_root_oob(content: &str) -> String {
    format!(r#"<div id="popup-root" hx-swap-oob="innerHTML">{content}</div>"#)
}

async fn render_rows_html(state: &AppState) -> Result<String, WebError> {
    let mut ctx = Context::new();
    ctx.insert("people", &load_list_items(state).await?);
    state
        .tera
        .render("people/list_rows.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")).into())
}

async fn load_list_items(
    state: &AppState,
) -> Result<Vec<ritmo_presenter::PersonListItem>, WebError> {
    Ok(ritmo_presenter::build_person_list_items(
        person::list_all_for_display(&state.core).await?,
    ))
}

fn parse_form_into_person(id: i64, form: &PersonFormData) -> Result<Person, WebError> {
    Ok(Person {
        id,
        name: form.name.trim().to_owned(),
        display_name: trim_to_option(form.display_name.clone()),
        given_name: trim_to_option(form.given_name.clone()),
        surname: trim_to_option(form.surname.clone()),
        middle_names: trim_to_option(form.middle_names.clone()),
        title: trim_to_option(form.title.clone()),
        suffix: trim_to_option(form.suffix.clone()),
        birth_date: parse_partial_date(
            form.birth_date_year.clone(),
            form.birth_date_month.clone(),
            form.birth_date_day.clone(),
            form.birth_date_circa,
        )?,
        death_date: parse_partial_date(
            form.death_date_year.clone(),
            form.death_date_month.clone(),
            form.death_date_day.clone(),
            form.death_date_circa,
        )?,
        biography: trim_to_option(form.biography.clone()),
        verified: false,
    })
}

fn empty_form_view() -> PersonFormView {
    PersonFormView {
        id: None,
        name: String::new(),
        display_name: None,
        given_name: None,
        surname: None,
        middle_names: None,
        title: None,
        suffix: None,
        biography: None,
        birth_date_year: None,
        birth_date_month: None,
        birth_date_day: None,
        birth_date_circa: false,
        death_date_year: None,
        death_date_month: None,
        death_date_day: None,
        death_date_circa: false,
    }
}

fn view_from_person(person: &Person) -> PersonFormView {
    PersonFormView {
        id: Some(person.id),
        name: person.name.clone(),
        display_name: person.display_name.clone(),
        given_name: person.given_name.clone(),
        surname: person.surname.clone(),
        middle_names: person.middle_names.clone(),
        title: person.title.clone(),
        suffix: person.suffix.clone(),
        biography: person.biography.clone(),
        birth_date_year: person.birth_date.as_ref().and_then(|d| d.year),
        birth_date_month: person.birth_date.as_ref().and_then(|d| d.month),
        birth_date_day: person.birth_date.as_ref().and_then(|d| d.day),
        birth_date_circa: person.birth_date.as_ref().is_some_and(|d| d.circa),
        death_date_year: person.death_date.as_ref().and_then(|d| d.year),
        death_date_month: person.death_date.as_ref().and_then(|d| d.month),
        death_date_day: person.death_date.as_ref().and_then(|d| d.day),
        death_date_circa: person.death_date.as_ref().is_some_and(|d| d.circa),
    }
}

fn form_into_view(id: Option<i64>, form: &PersonFormData) -> Result<PersonFormView, WebError> {
    let birth_date = parse_partial_date(
        form.birth_date_year.clone(),
        form.birth_date_month.clone(),
        form.birth_date_day.clone(),
        form.birth_date_circa,
    )?;
    let death_date = parse_partial_date(
        form.death_date_year.clone(),
        form.death_date_month.clone(),
        form.death_date_day.clone(),
        form.death_date_circa,
    )?;
    Ok(PersonFormView {
        id,
        name: form.name.clone(),
        display_name: trim_to_option(form.display_name.clone()),
        given_name: trim_to_option(form.given_name.clone()),
        surname: trim_to_option(form.surname.clone()),
        middle_names: trim_to_option(form.middle_names.clone()),
        title: trim_to_option(form.title.clone()),
        suffix: trim_to_option(form.suffix.clone()),
        biography: trim_to_option(form.biography.clone()),
        birth_date_year: birth_date.as_ref().and_then(|d| d.year),
        birth_date_month: birth_date.as_ref().and_then(|d| d.month),
        birth_date_day: birth_date.as_ref().and_then(|d| d.day),
        birth_date_circa: birth_date.as_ref().is_some_and(|d| d.circa),
        death_date_year: death_date.as_ref().and_then(|d| d.year),
        death_date_month: death_date.as_ref().and_then(|d| d.month),
        death_date_day: death_date.as_ref().and_then(|d| d.day),
        death_date_circa: death_date.as_ref().is_some_and(|d| d.circa),
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
    use super::{create, list, new_form, PersonFormData};
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
    async fn new_popup_disables_place_widget() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let html = new_form(State(app_state(core))).await.unwrap().0;
        assert!(html.contains("overlay visible"));
        assert!(html.contains("I widget relazionali saranno disponibili"));
    }

    #[tokio::test]
    async fn list_marks_people_active_and_wires_global_new() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let html = list(State(app_state(core))).await.unwrap().0;
        assert!(html.contains(r#"href="/people""#));
        assert!(html.contains(r#"class="view-toggle-link active""#));
        assert!(html.contains(r#"hx-get="/people/new""#));
    }

    #[tokio::test]
    async fn create_keeps_popup_open_and_enables_places_widget() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let state = app_state(core);
        let html = create(
            State(state),
            Form(PersonFormData {
                name: "Persona popup".to_owned(),
                display_name: None,
                given_name: None,
                surname: None,
                middle_names: None,
                title: None,
                suffix: None,
                biography: None,
                birth_date_year: None,
                birth_date_month: None,
                birth_date_day: None,
                birth_date_circa: false,
                death_date_year: None,
                death_date_month: None,
                death_date_day: None,
                death_date_circa: false,
            }),
        )
        .await
        .unwrap()
        .0;
        assert!(html.contains("hx-swap-oob"));
        assert!(html.contains("place-list-people-1"));
    }
}
