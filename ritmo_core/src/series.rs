use crate::CoreContext;
use ritmo_domain::Series;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::SeriesRepository;

pub async fn list_all(ctx: &CoreContext) -> RitmoResult<Vec<Series>> {
    SeriesRepository::new(&ctx.ctx).list_all().await
}

pub async fn create(ctx: &CoreContext, item: &Series) -> RitmoResult<i64> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = SeriesRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Series) -> RitmoResult<()> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = SeriesRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    let repo = SeriesRepository::new(&ctx.ctx);
    repo.delete(id).await
}
