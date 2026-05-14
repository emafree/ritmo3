use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::Alias;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct AliasRepository {
    pool: SqlitePool,
}

impl AliasRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, alias: &Alias) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT OR IGNORE INTO aliases(name, person_id) VALUES (?, ?)")
            .bind(&alias.alternative_name)
            .bind(alias.person_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Alias> {
        let row = sqlx::query("SELECT id, name, person_id FROM aliases WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(Alias {
            id: row.get("id"),
            alternative_name: row.get("name"),
            person_id: row.get("person_id"),
        })
    }

    pub async fn update(&self, alias: &Alias) -> RitmoResult<()> {
        sqlx::query("UPDATE aliases SET name = ?, person_id = ? WHERE id = ?")
            .bind(&alias.alternative_name)
            .bind(alias.person_id)
            .bind(alias.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM aliases WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Alias>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query(
            "SELECT id, name, person_id FROM aliases WHERE name LIKE ? COLLATE NOCASE ORDER BY name",
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Alias {
                id: row.get("id"),
                alternative_name: row.get("name"),
                person_id: row.get("person_id"),
            })
            .collect())
    }

    pub async fn list_by_person(&self, person_id: i64) -> RitmoResult<Vec<Alias>> {
        let rows = sqlx::query(
            "SELECT id, name, person_id FROM aliases WHERE person_id = ? ORDER BY name",
        )
        .bind(person_id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Alias {
                id: row.get("id"),
                alternative_name: row.get("name"),
                person_id: row.get("person_id"),
            })
            .collect())
    }

    pub async fn get_by_person_and_name(
        &self,
        person_id: i64,
        name: &str,
    ) -> RitmoResult<Option<Alias>> {
        let row =
            sqlx::query("SELECT id, name, person_id FROM aliases WHERE person_id = ? AND name = ?")
                .bind(person_id)
                .bind(name)
                .fetch_optional(&self.pool)
                .await
                .map_err(map_query)?;

        Ok(row.map(|value| Alias {
            id: value.get("id"),
            alternative_name: value.get("name"),
            person_id: value.get("person_id"),
        }))
    }
}
