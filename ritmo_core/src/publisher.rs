use crate::CoreContext;
use ritmo_domain::Publisher;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::PublisherRepository;

pub async fn get(ctx: &CoreContext, id: i64) -> RitmoResult<Publisher> {
    PublisherRepository::new(&ctx.ctx).get(id).await
}

pub async fn list_all(ctx: &CoreContext) -> RitmoResult<Vec<Publisher>> {
    PublisherRepository::new(&ctx.ctx).list_all().await
}

pub async fn search(ctx: &CoreContext, query: &str) -> RitmoResult<Vec<Publisher>> {
    PublisherRepository::new(&ctx.ctx).search(query.trim()).await
}

pub async fn get_or_create(ctx: &CoreContext, name: &str) -> RitmoResult<Publisher> {
    PublisherRepository::new(&ctx.ctx)
        .get_or_create(name.trim())
        .await
}

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
    let references = repo.is_referenced(id).await?;
    if references > 0 {
        return Err(RitmoErr::InvalidInput(format!(
            "Impossibile eliminare: è utilizzata da {references} record."
        )));
    }
    repo.delete(id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ritmo_repository::RepositoryContext;

    #[tokio::test]
    async fn delete_blocks_when_publisher_is_referenced() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let repo_ctx = RepositoryContext::new(pool);
        let core = CoreContext::new(repo_ctx.clone());

        let publisher_id = PublisherRepository::new(&repo_ctx)
            .save(&Publisher {
                id: 0,
                name: "Editore".to_owned(),
            })
            .await
            .unwrap();

        sqlx::query(
            "INSERT INTO d_books(name, publisher_id, has_cover, has_paper) VALUES (?, ?, 0, 0)",
        )
        .bind("Libro")
        .bind(publisher_id)
        .execute(repo_ctx.pool())
        .await
        .unwrap();

        let err = delete(&core, publisher_id).await.unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid input: Impossibile eliminare: è utilizzata da 1 record."
        );
    }
}
