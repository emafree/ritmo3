use crate::CoreContext;
use ritmo_domain::{Alias, Person, Place};
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    AliasRepository, PersonRepository, PlaceRepository, XBooksPeopleRolesRepository,
    XContentsPeopleRolesRepository, XPersonLanguagesRepository,
};

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
    let alias_repo = AliasRepository::new(&ctx.ctx);
    for alias in alias_repo.list_by_person(id).await? {
        alias_repo.delete(alias.id).await?;
    }

    let place_repo = PlaceRepository::new(&ctx.ctx);
    for place in place_repo.list_by_person(id).await? {
        place_repo.delete(place.id).await?;
    }

    let books_roles_repo = XBooksPeopleRolesRepository::new(&ctx.ctx);
    for (book_id, role_id) in books_roles_repo.list_by_person(id).await? {
        books_roles_repo.delete(book_id, id, role_id).await?;
    }

    let contents_roles_repo = XContentsPeopleRolesRepository::new(&ctx.ctx);
    for (content_id, role_id) in contents_roles_repo.list_by_person(id).await? {
        contents_roles_repo.delete(content_id, id, role_id).await?;
    }

    let person_langs_repo = XPersonLanguagesRepository::new(&ctx.ctx);
    for (language_id, role_id) in person_langs_repo.list_by_person(id).await? {
        person_langs_repo.delete(id, language_id, role_id).await?;
    }

    let person_repo = PersonRepository::new(&ctx.ctx);
    person_repo.delete(id).await
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
    if place.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = PlaceRepository::new(&ctx.ctx);
    repo.save(place).await
}

pub async fn remove_place(ctx: &CoreContext, place_id: i64) -> RitmoResult<()> {
    let repo = PlaceRepository::new(&ctx.ctx);
    repo.delete(place_id).await
}
