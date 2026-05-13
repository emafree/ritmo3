use crate::support::{map_delete, map_insert, map_query, RepositoryContext};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct XPublisherPlacesRepository {
    pool: SqlitePool,
}

impl XPublisherPlacesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(
        &self,
        publisher_id: i64,
        place_id: i64,
        place_type_id: i64,
    ) -> RitmoResult<()> {
        sqlx::query(
            "INSERT INTO x_publisher_places(publisher_id, place_id, place_type_id) VALUES (?, ?, ?)",
        )
        .bind(publisher_id)
        .bind(place_id)
        .bind(place_type_id)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(
        &self,
        publisher_id: i64,
        place_id: i64,
        place_type_id: i64,
    ) -> RitmoResult<()> {
        sqlx::query(
            "DELETE FROM x_publisher_places WHERE publisher_id = ? AND place_id = ? AND place_type_id = ?",
        )
        .bind(publisher_id)
        .bind(place_id)
        .bind(place_type_id)
        .execute(&self.pool)
        .await
        .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_publisher(&self, publisher_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = sqlx::query(
            "SELECT place_id, place_type_id FROM x_publisher_places WHERE publisher_id = ?",
        )
        .bind(publisher_id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| (row.get("place_id"), row.get("place_type_id")))
            .collect())
    }

    pub async fn list_by_place(&self, place_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = sqlx::query(
            "SELECT publisher_id, place_type_id FROM x_publisher_places WHERE place_id = ?",
        )
        .bind(place_id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| (row.get("publisher_id"), row.get("place_type_id")))
            .collect())
    }
}
