use crate::CoreContext;
use ritmo_domain::Format;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::FormatRepository;

pub async fn create(ctx: &CoreContext, item: &Format) -> RitmoResult<i64> {
    if item.i18n_key.trim().is_empty() {
        return Err(RitmoErr::InvalidInput(
            "i18n_key cannot be empty".to_string(),
        ));
    }
    let repo = FormatRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Format) -> RitmoResult<()> {
    if item.i18n_key.trim().is_empty() {
        return Err(RitmoErr::InvalidInput(
            "i18n_key cannot be empty".to_string(),
        ));
    }
    let repo = FormatRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    let repo = FormatRepository::new(&ctx.ctx);
    repo.delete(id).await
}
