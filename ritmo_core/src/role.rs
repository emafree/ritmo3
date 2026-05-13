use crate::CoreContext;
use ritmo_domain::Role;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{RoleRepository, XBooksPeopleRolesRepository, XContentsPeopleRolesRepository};

pub async fn create(ctx: &CoreContext, item: &Role) -> RitmoResult<i64> {
    if item.i18n_key.trim().is_empty() {
        return Err(RitmoErr::InvalidInput(
            "i18n_key cannot be empty".to_string(),
        ));
    }
    let repo = RoleRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Role) -> RitmoResult<()> {
    if item.i18n_key.trim().is_empty() {
        return Err(RitmoErr::InvalidInput(
            "i18n_key cannot be empty".to_string(),
        ));
    }
    let repo = RoleRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    let books_repo = XBooksPeopleRolesRepository::new(&ctx.ctx);
    let contents_repo = XContentsPeopleRolesRepository::new(&ctx.ctx);
    let book_links = books_repo.list_by_role(id).await?;
    let content_links = contents_repo.list_by_role(id).await?;
    if !book_links.is_empty() || !content_links.is_empty() {
        return Err(RitmoErr::DataIntegrity(
            "role is referenced and cannot be deleted".to_string(),
        ));
    }
    let repo = RoleRepository::new(&ctx.ctx);
    repo.delete(id).await
}
