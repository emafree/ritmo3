use crate::CoreContext;
use ritmo_domain::Language;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::LanguageRepository;

pub async fn list_all(ctx: &CoreContext) -> RitmoResult<Vec<Language>> {
    LanguageRepository::new(&ctx.ctx).list_all().await
}

pub async fn create(ctx: &CoreContext, item: &Language) -> RitmoResult<i64> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = LanguageRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Language) -> RitmoResult<()> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = LanguageRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    let repo = LanguageRepository::new(&ctx.ctx);
    let references = repo.is_referenced(id).await?;
    if references > 0 {
        return Err(RitmoErr::InvalidInput(format!(
            "Impossibile eliminare: è utilizzata da {references} record."
        )));
    }
    repo.delete(id).await
}
