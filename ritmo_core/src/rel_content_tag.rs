use crate::CoreContext;
use ritmo_errors::RitmoResult;
use ritmo_repository::XContentsTagsRepository;

pub async fn link(ctx: &CoreContext, content_id: i64, tag_id: i64) -> RitmoResult<()> {
    let repo = XContentsTagsRepository::new(&ctx.ctx);
    repo.create(content_id, tag_id).await
}

pub async fn unlink(ctx: &CoreContext, content_id: i64, tag_id: i64) -> RitmoResult<()> {
    let repo = XContentsTagsRepository::new(&ctx.ctx);
    repo.delete(content_id, tag_id).await
}
