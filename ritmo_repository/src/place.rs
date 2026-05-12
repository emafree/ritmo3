use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::Place;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct PlaceRepository {
    pool: SqlitePool,
}

impl PlaceRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, place: &Place) -> RitmoResult<i64> {
        let result = sqlx::query(
            "INSERT INTO person_places(person_id, place_type_id, place_name) VALUES (?, ?, ?)",
        )
        .bind(place.person_id)
        .bind(place.place_type_id)
        .bind(&place.name)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Place> {
        let row = sqlx::query(
            "SELECT id, place_name, place_type_id, person_id FROM person_places WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?
        .ok_or_else(not_found)?;

        Ok(Place {
            id: row.get("id"),
            name: row.get("place_name"),
            place_type_id: row.get("place_type_id"),
            person_id: row.get("person_id"),
        })
    }

    pub async fn update(&self, place: &Place) -> RitmoResult<()> {
        sqlx::query(
            "UPDATE person_places SET place_name = ?, place_type_id = ?, person_id = ? WHERE id = ?",
        )
        .bind(&place.name)
        .bind(place.place_type_id)
        .bind(place.person_id)
        .bind(place.id)
        .execute(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM person_places WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_person(&self, person_id: i64) -> RitmoResult<Vec<Place>> {
        let rows = sqlx::query(
            "SELECT id, place_name, place_type_id, person_id FROM person_places WHERE person_id = ? ORDER BY place_name",
        )
        .bind(person_id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| Place {
                id: row.get("id"),
                name: row.get("place_name"),
                place_type_id: row.get("place_type_id"),
                person_id: row.get("person_id"),
            })
            .collect())
    }
}
