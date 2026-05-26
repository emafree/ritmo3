use crate::support::{map_query, not_found, RepositoryContext};
use ritmo_domain::ContentType;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct ContentTypeRepository {
    pool: SqlitePool,
}

impl ContentTypeRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn get(&self, id: i64) -> RitmoResult<ContentType> {
        let row = sqlx::query("SELECT id, key FROM d_types WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;

        Ok(ContentType {
            id: row.get("id"),
            i18n_key: row.get("key"),
        })
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<ContentType>> {
        let rows = sqlx::query("SELECT id, key FROM d_types ORDER BY key")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| ContentType {
                id: row.get("id"),
                i18n_key: row.get("key"),
            })
            .collect())
    }

    pub async fn list_all_with_label(
        &self,
        language_code: &str,
    ) -> RitmoResult<Vec<(i64, String, String)>> {
        let rows = sqlx::query(
            "SELECT
                t.id,
                t.key,
                COALESCE(stt.name, t.key) AS label
             FROM d_types t
             LEFT JOIN s_type_translations stt
               ON stt.type_id = t.id
              AND stt.language_code = ?
             ORDER BY label COLLATE NOCASE",
        )
        .bind(language_code)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.get::<i64, _>("id"),
                    row.get::<String, _>("key"),
                    row.get::<String, _>("label"),
                )
            })
            .collect())
    }
}
