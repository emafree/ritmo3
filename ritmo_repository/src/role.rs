use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::Role;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct RoleRepository {
    pool: SqlitePool,
}

impl RoleRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, item: &Role) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT OR IGNORE INTO roles(key) VALUES (?)")
            .bind(&item.i18n_key)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Role> {
        let row = sqlx::query("SELECT id, key FROM roles WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(Role {
            id: row.get("id"),
            i18n_key: row.get("key"),
        })
    }

    pub async fn update(&self, item: &Role) -> RitmoResult<()> {
        sqlx::query("UPDATE roles SET key = ? WHERE id = ?")
            .bind(&item.i18n_key)
            .bind(item.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM roles WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Role>> {
        let rows = sqlx::query("SELECT id, key FROM roles ORDER BY key")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Role {
                id: row.get("id"),
                i18n_key: row.get("key"),
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Role>> {
        let pattern = format!("%{query}%");
        let rows =
            sqlx::query("SELECT id, key FROM roles WHERE key LIKE ? COLLATE NOCASE ORDER BY key")
                .bind(pattern)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Role {
                id: row.get("id"),
                i18n_key: row.get("key"),
            })
            .collect())
    }

    pub async fn get_or_create(&self, value: &str) -> RitmoResult<Role> {
        if let Some(row) = sqlx::query("SELECT id, key FROM roles WHERE key = ?")
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(Role {
                id: row.get("id"),
                i18n_key: row.get("key"),
            });
        }

        let created = Role {
            id: 0,
            i18n_key: value.to_string(),
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
