use crate::support::{map_delete, map_insert, map_query, RepositoryContext};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct XContentLanguagesRepository {
    pool: SqlitePool,
}

impl XContentLanguagesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(
        &self,
        content_id: i64,
        language_id: i64,
        language_role_id: i64,
    ) -> RitmoResult<()> {
        sqlx::query("INSERT OR IGNORE INTO x_content_languages(content_id, language_id, role_id) VALUES (?, ?, ?)")
            .bind(content_id)
            .bind(language_id)
            .bind(language_role_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(
        &self,
        content_id: i64,
        language_id: i64,
        language_role_id: i64,
    ) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_content_languages WHERE content_id = ? AND language_id = ? AND role_id = ?")
            .bind(content_id)
            .bind(language_id)
            .bind(language_role_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_content(&self, content_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows =
            sqlx::query("SELECT language_id, role_id FROM x_content_languages WHERE content_id = ?")
                .bind(content_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("language_id"), row.get("role_id")))
            .collect())
    }

    pub async fn list_by_language(
        &self,
        language_id: i64,
        language_role_id: Option<i64>,
    ) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = match language_role_id {
            Some(role_id) => {
                sqlx::query(
                    "SELECT content_id, role_id FROM x_content_languages WHERE language_id = ? AND role_id = ?",
                )
                .bind(language_id)
                .bind(role_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?
            }
            None => {
                sqlx::query(
                    "SELECT content_id, role_id FROM x_content_languages WHERE language_id = ?",
                )
                .bind(language_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?
            }
        };
        Ok(rows
            .into_iter()
            .map(|row| (row.get("content_id"), row.get("role_id")))
            .collect())
    }
}
