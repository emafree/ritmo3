use axum::extract::{Form, Path, Query, State};
use axum::response::Html;
use ritmo_core::place::{self, PlaceOwner};
use ritmo_domain::Place;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

#[derive(Debug, Clone, Copy)]
enum EntityRoute {
    People,
    Publishers,
}

impl EntityRoute {
    fn parse(value: &str) -> Result<Self, ritmo_errors::RitmoErr> {
        match value {
            "people" => Ok(Self::People),
            "publishers" => Ok(Self::Publishers),
            _ => Err(ritmo_errors::RitmoErr::InvalidInput(format!(
                "unknown entity_type: {value}"
            ))),
        }
    }

    fn owner(self, entity_id: i64) -> PlaceOwner {
        match self {
            Self::People => PlaceOwner::Person(entity_id),
            Self::Publishers => PlaceOwner::Publisher(entity_id),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PlacePanelQuery {
    entity_type: String,
    entity_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PlaceSearchQuery {
    q: Option<String>,
    entity_type: String,
    entity_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PlaceEditQuery {
    entity_type: String,
    entity_id: i64,
    place_type_key: String,
}

#[derive(Debug, Deserialize)]
pub struct LinkPlaceForm {
    place_id: i64,
    place_type_key: String,
}

#[derive(Debug, Deserialize)]
pub struct UpsertPlaceForm {
    continent: Option<String>,
    country: Option<String>,
    city: Option<String>,
    #[serde(default)]
    circa: bool,
    #[serde(default)]
    disputed: bool,
    entity_type: Option<String>,
    entity_id: Option<i64>,
    place_type_key: Option<String>,
    current_place_type_key: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SearchResultView {
    id: i64,
    display: String,
    continent: Option<String>,
    country: Option<String>,
    city: Option<String>,
    circa: bool,
    disputed: bool,
}

#[derive(Debug, Clone, Serialize)]
struct EditPlaceView {
    place_id: i64,
    continent: Option<String>,
    country: Option<String>,
    city: Option<String>,
    circa: bool,
    disputed: bool,
    place_type_key: String,
}

pub async fn search_panel(
    State(state): State<AppState>,
    Query(query): Query<PlacePanelQuery>,
) -> Result<Html<String>, WebError> {
    let entity_route = EntityRoute::parse(&query.entity_type)?;
    let html = render_search_panel(
        &state,
        &query.entity_type,
        query.entity_id,
        entity_route,
        Vec::new(),
        "",
        "other",
    )
    .await?;
    Ok(Html(html))
}

pub async fn search(
    State(state): State<AppState>,
    Query(query): Query<PlaceSearchQuery>,
) -> Result<Html<String>, WebError> {
    let _ = EntityRoute::parse(&query.entity_type)?;
    let q = query.q.unwrap_or_default();
    let matches = if q.trim().is_empty() {
        Vec::new()
    } else {
        place::search(&state.core, &q)
            .await?
            .into_iter()
            .map(|place| SearchResultView {
                id: place.id,
                display: ritmo_presenter::build_place_display(
                    place.continent.clone(),
                    place.country.clone(),
                    place.city.clone(),
                ),
                continent: place.continent,
                country: place.country,
                city: place.city,
                circa: place.circa,
                disputed: place.disputed,
            })
            .collect()
    };

    let mut ctx = Context::new();
    ctx.insert("entity_type", &query.entity_type);
    ctx.insert("entity_id", &query.entity_id);
    ctx.insert("results", &matches);
    let html = state
        .tera
        .render("widgets/place_search_result_items.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn edit_row(
    State(state): State<AppState>,
    Path(place_id): Path<i64>,
    Query(query): Query<PlaceEditQuery>,
) -> Result<Html<String>, WebError> {
    let entity_route = EntityRoute::parse(&query.entity_type)?;
    let place = place::get(&state.core, place_id).await?;
    let mut ctx = Context::new();
    ctx.insert("entity_type", &query.entity_type);
    ctx.insert("entity_id", &query.entity_id);
    ctx.insert("place", &EditPlaceView {
        place_id: place.id,
        continent: place.continent,
        country: place.country,
        city: place.city,
        circa: place.circa,
        disputed: place.disputed,
        place_type_key: query.place_type_key,
    });
    ctx.insert(
        "place_types",
        &load_place_type_options(&state, entity_route, query.entity_id).await?,
    );
    let html = state
        .tera
        .render("widgets/place_row_edit.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

pub async fn update(
    State(state): State<AppState>,
    Path(place_id): Path<i64>,
    Form(form): Form<UpsertPlaceForm>,
) -> Result<Html<String>, WebError> {
    let place = Place {
        id: place_id,
        continent: form.continent,
        country: form.country,
        city: form.city,
        circa: form.circa,
        disputed: form.disputed,
    };
    place::update(&state.core, &place).await?;

    let entity_type = form
        .entity_type
        .ok_or_else(|| ritmo_errors::RitmoErr::InvalidInput("missing entity_type".to_owned()))?;
    let entity_id = form
        .entity_id
        .ok_or_else(|| ritmo_errors::RitmoErr::InvalidInput("missing entity_id".to_owned()))?;
    let place_type_key = form
        .place_type_key
        .ok_or_else(|| ritmo_errors::RitmoErr::InvalidInput("missing place_type_key".to_owned()))?;
    let current_place_type_key = form.current_place_type_key.unwrap_or_else(|| place_type_key.clone());

    let entity_route = EntityRoute::parse(&entity_type)?;
    place::replace_link_type(
        &state.core,
        entity_route.owner(entity_id),
        place_id,
        &current_place_type_key,
        &place_type_key,
    )
    .await?;
    render_linked_place_row(&state, &entity_type, entity_id, place_id, &place_type_key).await
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<UpsertPlaceForm>,
) -> Result<Html<String>, WebError> {
    let place_id = place::create(
        &state.core,
        &Place {
            id: 0,
            continent: form.continent,
            country: form.country,
            city: form.city,
            circa: form.circa,
            disputed: form.disputed,
        },
    )
    .await?;

    if let (Some(entity_type), Some(entity_id), Some(place_type_key)) =
        (form.entity_type, form.entity_id, form.place_type_key)
    {
        let entity_route = EntityRoute::parse(&entity_type)?;
        place::link(
            &state.core,
            entity_route.owner(entity_id),
            place_id,
            &place_type_key,
        )
        .await?;
        return render_linked_place_row(&state, &entity_type, entity_id, place_id, &place_type_key)
            .await;
    }

    Ok(Html(String::new()))
}

pub async fn link(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, i64)>,
    Form(form): Form<LinkPlaceForm>,
) -> Result<Html<String>, WebError> {
    let entity_route = EntityRoute::parse(&entity_type)?;
    place::link(
        &state.core,
        entity_route.owner(entity_id),
        form.place_id,
        &form.place_type_key,
    )
    .await?;

    render_linked_place_row(
        &state,
        &entity_type,
        entity_id,
        form.place_id,
        &form.place_type_key,
    )
    .await
}

pub async fn unlink(
    State(state): State<AppState>,
    Path((entity_type, entity_id, place_id)): Path<(String, i64, i64)>,
) -> Result<Html<String>, WebError> {
    let entity_route = EntityRoute::parse(&entity_type)?;
    place::unlink(&state.core, entity_route.owner(entity_id), place_id).await?;
    Ok(Html(String::new()))
}

async fn render_search_panel(
    state: &AppState,
    entity_type: &str,
    entity_id: i64,
    entity_route: EntityRoute,
    results: Vec<SearchResultView>,
    query: &str,
    place_type_key: &str,
) -> Result<String, WebError> {
    let mut ctx = Context::new();
    ctx.insert("entity_type", entity_type);
    ctx.insert("entity_id", &entity_id);
    ctx.insert("results", &results);
    ctx.insert("query", &query);
    ctx.insert("selected_place_type_key", &place_type_key);
    ctx.insert(
        "place_types",
        &load_place_type_options(state, entity_route, entity_id).await?,
    );
    state
        .tera
        .render("widgets/place_search_results.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")).into())
}

async fn render_linked_place_row(
    state: &AppState,
    entity_type: &str,
    entity_id: i64,
    place_id: i64,
    place_type_key: &str,
) -> Result<Html<String>, WebError> {
    let rows = place::list_linked(&state.core, EntityRoute::parse(entity_type)?.owner(entity_id))
        .await?;
    let row = rows
        .into_iter()
        .find(|row| row.place_id == place_id && row.place_type_key == place_type_key)
        .ok_or(ritmo_errors::RitmoErr::RecordNotFound)?;

    let rendered = ritmo_presenter::build_place_items(
        vec![(
            row.place_id,
            row.continent,
            row.country,
            row.city,
            row.circa,
            row.disputed,
            row.place_type_key,
        )],
        "it",
    );

    let mut ctx = Context::new();
    ctx.insert("entity_type", entity_type);
    ctx.insert("entity_id", &entity_id);
    ctx.insert("place", &rendered[0]);
    let html = state
        .tera
        .render("widgets/place_row.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))?;
    Ok(Html(html))
}

async fn load_place_type_options(
    state: &AppState,
    _entity_route: EntityRoute,
    _entity_id: i64,
) -> Result<Vec<ritmo_presenter::PlaceTypeOption>, WebError> {
    let keys = place::list_types(&state.core)
        .await?
        .into_iter()
        .map(|place_type| place_type.name)
        .collect();
    Ok(ritmo_presenter::build_place_type_options(keys, "it"))
}

#[cfg(test)]
mod tests {
    use super::{search, EntityRoute, PlaceSearchQuery};
    use crate::state::{load_tera, AppConfig, AppState};
    use axum::extract::{Query, State};
    use ritmo_core::person;
    use ritmo_core::place::{self, PlaceOwner};
    use ritmo_core::CoreContext;
    use ritmo_domain::{Person, Place};
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
    fn parse_entity_route_rejects_unknown_values() {
        assert!(matches!(EntityRoute::parse("people"), Ok(EntityRoute::People)));
        assert!(EntityRoute::parse("books").is_err());
    }

    #[tokio::test]
    async fn search_renders_matching_places() {
        let core = CoreContext::connect("sqlite::memory:").await.unwrap();
        let state = app_state(core.clone());

        let person_id = person::create(
            &core,
            &Person {
                id: 0,
                name: "Persona".to_owned(),
                display_name: None,
                given_name: None,
                surname: None,
                middle_names: None,
                title: None,
                suffix: None,
                birth_date: None,
                death_date: None,
                biography: None,
                verified: false,
            },
        )
        .await
        .unwrap();

        let place_id = place::create(
            &core,
            &Place {
                id: 0,
                continent: Some("Europa".to_owned()),
                country: Some("Italia".to_owned()),
                city: Some("Roma".to_owned()),
                circa: false,
                disputed: false,
            },
        )
        .await
        .unwrap();
        place::link(&core, PlaceOwner::Person(person_id), place_id, "birth")
            .await
            .unwrap();

        let html = search(
            State(state),
            Query(PlaceSearchQuery {
                q: Some("rom".to_owned()),
                entity_type: "people".to_owned(),
                entity_id: person_id,
            }),
        )
        .await
        .unwrap()
        .0;

        assert!(html.contains("Roma, Italia, Europa"));
    }
}
