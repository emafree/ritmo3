use crate::support::{map_delete, map_insert, map_query, RepositoryContext};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct XContentsTagsRepository {
    pool: SqlitePool,
}

impl XContentsTagsRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, content_id: i64, tag_id: i64) -> RitmoResult<()> {
        sqlx::query("INSERT INTO x_contents_tags(content_id, tag_id) VALUES (?, ?)")
            .bind(content_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, content_id: i64, tag_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_contents_tags WHERE content_id = ? AND tag_id = ?")
            .bind(content_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_content(&self, content_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT tag_id FROM x_contents_tags WHERE content_id = ?")
            .bind(content_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("tag_id")).collect())
    }

    pub async fn list_by_tag(&self, tag_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT content_id FROM x_contents_tags WHERE tag_id = ?")
            .bind(tag_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("content_id")).collect())
    }
}
