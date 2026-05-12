use crate::CoreContext;
use ritmo_errors::RitmoResult;
use ritmo_repository::XBooksContentsRepository;

pub async fn link(ctx: &CoreContext, book_id: i64, content_id: i64) -> RitmoResult<()> {
    let repo = XBooksContentsRepository::new(&ctx.ctx);
    repo.create(book_id, content_id).await
}

pub async fn unlink(ctx: &CoreContext, book_id: i64, content_id: i64) -> RitmoResult<()> {
    let repo = XBooksContentsRepository::new(&ctx.ctx);
    repo.delete(book_id, content_id).await
}
