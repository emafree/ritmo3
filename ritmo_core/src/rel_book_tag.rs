use crate::CoreContext;
use ritmo_errors::RitmoResult;
use ritmo_repository::XBooksTagsRepository;

pub async fn link(ctx: &CoreContext, book_id: i64, tag_id: i64) -> RitmoResult<()> {
    let repo = XBooksTagsRepository::new(&ctx.ctx);
    repo.create(book_id, tag_id).await
}

pub async fn unlink(ctx: &CoreContext, book_id: i64, tag_id: i64) -> RitmoResult<()> {
    let repo = XBooksTagsRepository::new(&ctx.ctx);
    repo.delete(book_id, tag_id).await
}
