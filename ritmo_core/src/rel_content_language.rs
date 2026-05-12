use crate::CoreContext;
use ritmo_errors::RitmoResult;
use ritmo_repository::XContentLanguagesRepository;

pub async fn link(
    ctx: &CoreContext,
    content_id: i64,
    language_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XContentLanguagesRepository::new(&ctx.ctx);
    repo.create(content_id, language_id, role_id).await
}

pub async fn unlink(
    ctx: &CoreContext,
    content_id: i64,
    language_id: i64,
    role_id: i64,
) -> RitmoResult<()> {
    let repo = XContentLanguagesRepository::new(&ctx.ctx);
    repo.delete(content_id, language_id, role_id).await
}
