use crate::CoreContext;
use ritmo_domain::ContentType;
use ritmo_errors::RitmoResult;
use ritmo_repository::ContentTypeRepository;

pub async fn get(ctx: &CoreContext, id: i64) -> RitmoResult<ContentType> {
    ContentTypeRepository::new(&ctx.ctx).get(id).await
}

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

pub async fn search(ctx: &CoreContext, query: &str) -> RitmoResult<Vec<ContentType>> {
    ContentTypeRepository::new(&ctx.ctx).search(query.trim()).await
}

pub async fn get_or_create(ctx: &CoreContext, key: &str) -> RitmoResult<ContentType> {
    ContentTypeRepository::new(&ctx.ctx)
        .get_or_create(key.trim())
        .await
}
