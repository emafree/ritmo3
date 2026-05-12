use crate::CoreContext;
use ritmo_domain::Language;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{
    LanguageRepository, XBookLanguagesRepository, XContentLanguagesRepository,
    XPersonLanguagesRepository,
};

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
    let book_langs = XBookLanguagesRepository::new(&ctx.ctx);
    let content_langs = XContentLanguagesRepository::new(&ctx.ctx);
    let person_langs = XPersonLanguagesRepository::new(&ctx.ctx);
    let book_links = book_langs.list_by_language(id, None).await?;
    let content_links = content_langs.list_by_language(id, None).await?;
    let person_links = person_langs.list_by_language(id, None).await?;
    if !book_links.is_empty() || !content_links.is_empty() || !person_links.is_empty() {
        return Err(RitmoErr::DataIntegrity(
            "language is referenced and cannot be deleted".to_string(),
        ));
    }
    let repo = LanguageRepository::new(&ctx.ctx);
    repo.delete(id).await
}
