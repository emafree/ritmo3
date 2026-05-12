use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::Publisher;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct PublisherRepository {
    pool: SqlitePool,
}

impl PublisherRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, item: &Publisher) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT INTO publishers(name) VALUES (?)")
            .bind(&item.name)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Publisher> {
        let row = sqlx::query("SELECT id, name FROM publishers WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(Publisher {
            id: row.get("id"),
            name: row.get("name"),
        })
    }

    pub async fn update(&self, item: &Publisher) -> RitmoResult<()> {
        sqlx::query("UPDATE publishers SET name = ? WHERE id = ?")
            .bind(&item.name)
            .bind(item.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM publishers WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Publisher>> {
        let rows = sqlx::query("SELECT id, name FROM publishers ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Publisher {
                id: row.get("id"),
                name: row.get("name"),
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Publisher>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query(
            "SELECT id, name FROM publishers WHERE name LIKE ? COLLATE NOCASE ORDER BY name",
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Publisher {
                id: row.get("id"),
                name: row.get("name"),
            })
            .collect())
    }

    pub async fn get_or_create(&self, value: &str) -> RitmoResult<Publisher> {
        if let Some(row) = sqlx::query("SELECT id, name FROM publishers WHERE name = ?")
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(Publisher {
                id: row.get("id"),
                name: row.get("name"),
            });
        }

        let created = Publisher {
            id: 0,
            name: value.to_string(),
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
