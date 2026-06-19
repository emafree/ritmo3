use crate::CoreContext;
use ritmo_domain::Format;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::FormatRepository;

pub async fn get(ctx: &CoreContext, id: i64) -> RitmoResult<Format> {
    FormatRepository::new(&ctx.ctx).get(id).await
}

pub async fn list_all(ctx: &CoreContext) -> RitmoResult<Vec<Format>> {
    FormatRepository::new(&ctx.ctx).list_all().await
}

pub async fn list_all_with_label(
    ctx: &CoreContext,
    language_code: &str,
) -> RitmoResult<Vec<(i64, String, String)>> {
    FormatRepository::new(&ctx.ctx)
        .list_all_with_label(language_code)
        .await
}

pub async fn search(ctx: &CoreContext, query: &str) -> RitmoResult<Vec<Format>> {
    FormatRepository::new(&ctx.ctx).search(query.trim()).await
}

pub async fn get_or_create(ctx: &CoreContext, key: &str) -> RitmoResult<Format> {
    FormatRepository::new(&ctx.ctx)
        .get_or_create(key.trim())
        .await
}

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
    let references = repo.is_referenced(id).await?;
    if references > 0 {
        return Err(RitmoErr::InvalidInput(format!(
            "Impossibile eliminare: è utilizzata da {references} record."
        )));
    }
    repo.delete(id).await
}
