use crate::CoreContext;
use ritmo_domain::{Alias, Person, Place};
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{AliasRepository, PersonRepository, PlaceRepository};

pub async fn create(ctx: &CoreContext, item: &Person) -> RitmoResult<i64> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = PersonRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Person) -> RitmoResult<()> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = PersonRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    PersonRepository::new(&ctx.ctx).delete(id).await
}

pub async fn add_alias(ctx: &CoreContext, alias: &Alias) -> RitmoResult<i64> {
    if alias.alternative_name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput(
            "alternative_name cannot be empty".to_string(),
        ));
    }
    let repo = AliasRepository::new(&ctx.ctx);
    repo.save(alias).await
}

pub async fn remove_alias(ctx: &CoreContext, alias_id: i64) -> RitmoResult<()> {
    let repo = AliasRepository::new(&ctx.ctx);
    repo.delete(alias_id).await
}

pub async fn add_place(ctx: &CoreContext, place: &Place) -> RitmoResult<i64> {
    let has_value = place
        .continent
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
        || place
            .country
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
        || place
            .city
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());

    if !has_value {
        return Err(RitmoErr::InvalidInput(
            "place must define continent, country, or city".to_string(),
        ));
    }
    let repo = PlaceRepository::new(&ctx.ctx);
    repo.save(place).await
}

pub async fn remove_place(ctx: &CoreContext, place_id: i64) -> RitmoResult<()> {
    let repo = PlaceRepository::new(&ctx.ctx);
    repo.delete(place_id).await
}
