use crate::CoreContext;
use ritmo_domain::Role;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::RoleRepository;

pub async fn list_all(ctx: &CoreContext) -> RitmoResult<Vec<Role>> {
    RoleRepository::new(&ctx.ctx).list_all().await
}

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
    let repo = RoleRepository::new(&ctx.ctx);
    let references = repo.is_referenced(id).await?;
    if references > 0 {
        return Err(RitmoErr::InvalidInput(format!(
            "Impossibile eliminare: è utilizzata da {references} record."
        )));
    }
    repo.delete(id).await
}
