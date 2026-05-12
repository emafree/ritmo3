use crate::support::{map_delete, map_insert, map_query, RepositoryContext};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct XBooksContentsRepository {
    pool: SqlitePool,
}

impl XBooksContentsRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, book_id: i64, content_id: i64) -> RitmoResult<()> {
        sqlx::query("INSERT INTO x_books_contents(book_id, content_id) VALUES (?, ?)")
            .bind(book_id)
            .bind(content_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, book_id: i64, content_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_books_contents WHERE book_id = ? AND content_id = ?")
            .bind(book_id)
            .bind(content_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_book(&self, book_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT content_id FROM x_books_contents WHERE book_id = ?")
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("content_id")).collect())
    }

    pub async fn list_by_content(&self, content_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT book_id FROM x_books_contents WHERE content_id = ?")
            .bind(content_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("book_id")).collect())
    }
}
