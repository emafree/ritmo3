use crate::CoreContext;
use ritmo_errors::RitmoResult;
use ritmo_repository::XBookLanguagesRepository;

pub async fn link(
    ctx: &CoreContext,
    book_id: i64,
    language_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XBookLanguagesRepository::new(&ctx.ctx);
    repo.create(book_id, language_id, role_id).await
}

pub async fn unlink(
    ctx: &CoreContext,
    book_id: i64,
    language_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XBookLanguagesRepository::new(&ctx.ctx);
    repo.delete(book_id, language_id, role_id).await
}
