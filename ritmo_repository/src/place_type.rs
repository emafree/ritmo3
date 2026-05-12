use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::PlaceType;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct PlaceTypeRepository {
    pool: SqlitePool,
}

impl PlaceTypeRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, place_type: &PlaceType) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT INTO person_place_types(key) VALUES (?)")
            .bind(&place_type.name)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<PlaceType> {
        let row = sqlx::query("SELECT id, key FROM person_place_types WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(PlaceType {
            id: row.get("id"),
            name: row.get("key"),
        })
    }

    pub async fn update(&self, place_type: &PlaceType) -> RitmoResult<()> {
        sqlx::query("UPDATE person_place_types SET key = ? WHERE id = ?")
            .bind(&place_type.name)
            .bind(place_type.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM person_place_types WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<PlaceType>> {
        let rows = sqlx::query("SELECT id, key FROM person_place_types ORDER BY key")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| PlaceType {
                id: row.get("id"),
                name: row.get("key"),
            })
            .collect())
    }
}
