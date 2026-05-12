use crate::CoreContext;
use ritmo_domain::Publisher;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::PublisherRepository;

pub async fn create(ctx: &CoreContext, item: &Publisher) -> RitmoResult<i64> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = PublisherRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Publisher) -> RitmoResult<()> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = PublisherRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    let repo = PublisherRepository::new(&ctx.ctx);
    repo.delete(id).await
}
