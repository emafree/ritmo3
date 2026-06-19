use crate::support::{map_insert, map_query, not_found, RepositoryContext};
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

    pub async fn save(&self, item: &ContentType) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT OR IGNORE INTO d_types(key) VALUES (?)")
            .bind(&item.i18n_key)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
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

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<ContentType>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query(
            "SELECT id, key FROM d_types WHERE key LIKE ? COLLATE NOCASE ORDER BY key",
        )
        .bind(pattern)
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

    pub async fn get_or_create(&self, value: &str) -> RitmoResult<ContentType> {
        let value = value.trim();

        if let Some(row) = sqlx::query("SELECT id, key FROM d_types WHERE TRIM(key) = ? COLLATE NOCASE")
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(ContentType {
                id: row.get("id"),
                i18n_key: row.get("key"),
            });
        }

        let created = ContentType {
            id: 0,
            i18n_key: value.to_string(),
        };
        let id = self.save(&created).await?;
        if id > 0 {
            return self.get(id).await;
        }

        let row = sqlx::query("SELECT id, key FROM d_types WHERE TRIM(key) = ? COLLATE NOCASE")
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(ContentType {
            id: row.get("id"),
            i18n_key: row.get("key"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RepositoryContext;

    #[tokio::test]
    async fn get_or_create_reuses_existing_type_case_insensitively() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let ctx = RepositoryContext::new(pool);
        let repo = ContentTypeRepository::new(&ctx);
        let existing_id = repo
            .save(&ContentType {
                id: 0,
                i18n_key: "novel".to_owned(),
            })
            .await
            .unwrap();

        let content_type = repo.get_or_create(" NOVEL ").await.unwrap();

        assert_eq!(content_type.id, existing_id);
        assert_eq!(content_type.i18n_key, "novel");
    }
}
