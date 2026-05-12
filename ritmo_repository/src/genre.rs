use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::Genre;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct GenreRepository {
    pool: SqlitePool,
}

impl GenreRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, item: &Genre) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT INTO genres(key) VALUES (?)")
            .bind(&item.i18n_key)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Genre> {
        let row = sqlx::query("SELECT id, key FROM genres WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(Genre {
            id: row.get("id"),
            i18n_key: row.get("key"),
        })
    }

    pub async fn update(&self, item: &Genre) -> RitmoResult<()> {
        sqlx::query("UPDATE genres SET key = ? WHERE id = ?")
            .bind(&item.i18n_key)
            .bind(item.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM genres WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Genre>> {
        let rows = sqlx::query("SELECT id, key FROM genres ORDER BY key")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Genre {
                id: row.get("id"),
                i18n_key: row.get("key"),
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Genre>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query(
            "SELECT id, key FROM genres WHERE key LIKE ? COLLATE NOCASE ORDER BY key",
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Genre {
                id: row.get("id"),
                i18n_key: row.get("key"),
            })
            .collect())
    }

    pub async fn get_or_create(&self, value: &str) -> RitmoResult<Genre> {
        if let Some(row) = sqlx::query("SELECT id, key FROM genres WHERE key = ?")
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(Genre {
                id: row.get("id"),
                i18n_key: row.get("key"),
            });
        }

        let created = Genre {
            id: 0,
            i18n_key: value.to_string(),
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
