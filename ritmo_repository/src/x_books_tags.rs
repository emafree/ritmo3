use crate::support::{map_delete, map_insert, map_query, RepositoryContext};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct XBooksTagsRepository {
    pool: SqlitePool,
}

impl XBooksTagsRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, book_id: i64, tag_id: i64) -> RitmoResult<()> {
        sqlx::query("INSERT OR IGNORE INTO x_books_tags(book_id, tag_id) VALUES (?, ?)")
            .bind(book_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, book_id: i64, tag_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_books_tags WHERE book_id = ? AND tag_id = ?")
            .bind(book_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_book(&self, book_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT tag_id FROM x_books_tags WHERE book_id = ?")
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("tag_id")).collect())
    }

    pub async fn list_by_tag(&self, tag_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT book_id FROM x_books_tags WHERE tag_id = ?")
            .bind(tag_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("book_id")).collect())
    }
}
