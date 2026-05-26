use crate::CoreContext;
use ritmo_domain::ContentType;
use ritmo_errors::RitmoResult;
use ritmo_repository::ContentTypeRepository;

pub async fn list_all(ctx: &CoreContext) -> RitmoResult<Vec<ContentType>> {
    ContentTypeRepository::new(&ctx.ctx).list_all().await
}

pub async fn list_all_with_label(
    ctx: &CoreContext,
    language_code: &str,
) -> RitmoResult<Vec<(i64, String, String)>> {
    ContentTypeRepository::new(&ctx.ctx)
        .list_all_with_label(language_code)
        .await
}
