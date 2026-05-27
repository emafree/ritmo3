use axum::extract::{Path, State};
use axum::response::Html;
use ritmo_presenter::{build_publisher_detail, build_publisher_list_items, PublisherDetail};
use ritmo_repository::{PublisherDetailData, PublisherRepository};
use tera::Context;

use crate::error::WebError;
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) -> Result<Html<String>, WebError> {
    let rows = PublisherRepository::new(&state.repo).list_all().await?;
    let publishers =
        build_publisher_list_items(rows.into_iter().map(|row| (row.id, row.name)).collect());

    let mut ctx = Context::new();
    ctx.insert("publishers", &publishers);

    let body = state
        .tera
        .render("publishers/list.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, WebError> {
    let detail = PublisherRepository::new(&state.repo).get_detail(id).await?;
    let publisher = build_publisher_detail_vm(detail);

    let mut ctx = Context::new();
    ctx.insert("publisher", &publisher);

    let body = state
        .tera
        .render("publishers/detail.html", &ctx)
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(e.to_string()))?;

    Ok(Html(body))
}

fn build_publisher_detail_vm(detail: PublisherDetailData) -> PublisherDetail {
    build_publisher_detail(
        detail.id,
        detail.name,
        detail.country,
        detail.website,
        detail.notes,
        detail.places,
    )
}
