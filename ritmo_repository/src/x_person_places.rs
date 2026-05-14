use crate::support::{map_delete, map_insert, map_query, RepositoryContext};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct XPersonPlacesRepository {
    pool: SqlitePool,
}

impl XPersonPlacesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(
        &self,
        person_id: i64,
        place_id: i64,
        place_type_id: i64,
    ) -> RitmoResult<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO x_person_places(person_id, place_id, place_type_id) VALUES (?, ?, ?)",
        )
        .bind(person_id)
        .bind(place_id)
        .bind(place_type_id)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(
        &self,
        person_id: i64,
        place_id: i64,
        place_type_id: i64,
    ) -> RitmoResult<()> {
        sqlx::query(
            "DELETE FROM x_person_places WHERE person_id = ? AND place_id = ? AND place_type_id = ?",
        )
        .bind(person_id)
        .bind(place_id)
        .bind(place_type_id)
        .execute(&self.pool)
        .await
        .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_person(&self, person_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows =
            sqlx::query("SELECT place_id, place_type_id FROM x_person_places WHERE person_id = ?")
                .bind(person_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| (row.get("place_id"), row.get("place_type_id")))
            .collect())
    }

    pub async fn list_by_place(&self, place_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows =
            sqlx::query("SELECT person_id, place_type_id FROM x_person_places WHERE place_id = ?")
                .bind(place_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| (row.get("person_id"), row.get("place_type_id")))
            .collect())
    }
}
