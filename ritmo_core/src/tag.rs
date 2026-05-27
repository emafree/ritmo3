use crate::CoreContext;
use ritmo_domain::Tag;
use ritmo_errors::{RitmoErr, RitmoResult};
use ritmo_repository::TagRepository;

pub async fn list_all(ctx: &CoreContext) -> RitmoResult<Vec<Tag>> {
    TagRepository::new(&ctx.ctx).list_all().await
}

pub async fn create(ctx: &CoreContext, item: &Tag) -> RitmoResult<i64> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = TagRepository::new(&ctx.ctx);
    repo.save(item).await
}

pub async fn update(ctx: &CoreContext, item: &Tag) -> RitmoResult<()> {
    if item.name.trim().is_empty() {
        return Err(RitmoErr::InvalidInput("name cannot be empty".to_string()));
    }
    let repo = TagRepository::new(&ctx.ctx);
    repo.update(item).await
}

pub async fn delete(ctx: &CoreContext, id: i64) -> RitmoResult<()> {
    TagRepository::new(&ctx.ctx).delete(id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ritmo_domain::Book;
    use ritmo_repository::{BookRepository, RepositoryContext, XBooksTagsRepository};

    #[tokio::test]
    async fn delete_allows_referenced_tag() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let repo_ctx = RepositoryContext::new(pool);
        let core = CoreContext::new(repo_ctx.clone());

        let book_id = BookRepository::new(&repo_ctx)
            .save(&Book {
                id: 0,
                title: "Libro".to_owned(),
                original_title: None,
                publisher_id: None,
                format_id: None,
                series_id: None,
                series_index: None,
                isbn: None,
                publication_year: None,
                notes: None,
                has_cover: false,
                has_paper: false,
            })
            .await
            .unwrap();
        let tag_id = TagRepository::new(&repo_ctx)
            .save(&Tag {
                id: 0,
                name: "Fantascienza".to_owned(),
                tag_type: "genre".to_owned(),
            })
            .await
            .unwrap();
        XBooksTagsRepository::new(&repo_ctx)
            .save(book_id, tag_id)
            .await
            .unwrap();

        delete(&core, tag_id).await.unwrap();

        assert!(TagRepository::new(&repo_ctx).get(tag_id).await.is_err());
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM x_books_tags WHERE tag_id = ?")
                .bind(tag_id)
                .fetch_one(repo_ctx.pool())
                .await
                .unwrap(),
            0
        );
    }
}
