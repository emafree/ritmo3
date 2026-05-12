use crate::CoreContext;
use ritmo_domain::Tag;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::{TagRepository, XBooksTagsRepository, XContentsTagsRepository};

pub async fn create(ctx: &CoreContext, item: &Tag) -> RitmoResult<i64> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = TagRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Tag) -> RitmoResult<()> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = TagRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    let books_repo = XBooksTagsRepository::new(&ctx.ctx);
    let contents_repo = XContentsTagsRepository::new(&ctx.ctx);
    let books = books_repo.list_by_tag(id).await?;
    let contents = contents_repo.list_by_tag(id).await?;
    if !books.is_empty() || !contents.is_empty() {
        return Err(RitmoErr::DataIntegrity(
            "tag is referenced and cannot be deleted".to_string(),
        ));
    }
    let repo = TagRepository::new(&ctx.ctx);
    repo.delete(id).await
}
