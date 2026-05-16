use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::Tag;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct TagRepository {
    pool: SqlitePool,
}

impl TagRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, item: &Tag) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT OR IGNORE INTO d_tags(name) VALUES (?)")
            .bind(&item.name)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Tag> {
        let row = sqlx::query("SELECT id, name FROM d_tags WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(Tag {
            id: row.get("id"),
            name: row.get("name"),
        })
    }

    pub async fn update(&self, item: &Tag) -> RitmoResult<()> {
        sqlx::query("UPDATE d_tags SET name = ? WHERE id = ?")
            .bind(&item.name)
            .bind(item.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM d_tags WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Tag>> {
        let rows = sqlx::query("SELECT id, name FROM d_tags ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Tag {
                id: row.get("id"),
                name: row.get("name"),
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Tag>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query(
            "SELECT id, name FROM d_tags WHERE name LIKE ? COLLATE NOCASE ORDER BY name",
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Tag {
                id: row.get("id"),
                name: row.get("name"),
            })
            .collect())
    }

    pub async fn get_or_create(&self, value: &str) -> RitmoResult<Tag> {
        if let Some(row) = sqlx::query("SELECT id, name FROM d_tags WHERE name = ?")
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(Tag {
                id: row.get("id"),
                name: row.get("name"),
            });
        }

        let created = Tag {
            id: 0,
            name: value.to_string(),
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
