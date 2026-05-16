use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::Series;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct SeriesRepository {
    pool: SqlitePool,
}

impl SeriesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, item: &Series) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT OR IGNORE INTO d_series(name) VALUES (?)")
            .bind(&item.name)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Series> {
        let row = sqlx::query("SELECT id, name FROM d_series WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(Series {
            id: row.get("id"),
            name: row.get("name"),
        })
    }

    pub async fn update(&self, item: &Series) -> RitmoResult<()> {
        sqlx::query("UPDATE d_series SET name = ? WHERE id = ?")
            .bind(&item.name)
            .bind(item.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM d_series WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Series>> {
        let rows = sqlx::query("SELECT id, name FROM d_series ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Series {
                id: row.get("id"),
                name: row.get("name"),
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Series>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query(
            "SELECT id, name FROM d_series WHERE name LIKE ? COLLATE NOCASE ORDER BY name",
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Series {
                id: row.get("id"),
                name: row.get("name"),
            })
            .collect())
    }

    pub async fn get_or_create(&self, value: &str) -> RitmoResult<Series> {
        if let Some(row) = sqlx::query("SELECT id, name FROM d_series WHERE name = ?")
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(Series {
                id: row.get("id"),
                name: row.get("name"),
            });
        }

        let created = Series {
            id: 0,
            name: value.to_string(),
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
