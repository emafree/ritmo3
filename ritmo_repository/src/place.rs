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
            "INSERT OR IGNORE INTO d_places(continent, country, city, circa, disputed) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&place.continent)
        .bind(&place.country)
        .bind(&place.city)
        .bind(i64::from(place.circa))
        .bind(i64::from(place.disputed))
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Place> {
        let row = sqlx::query(
            "SELECT id, continent, country, city, circa, disputed FROM d_places WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?
        .ok_or_else(not_found)?;

        Ok(Self::map_place(row))
    }

    pub async fn update(&self, place: &Place) -> RitmoResult<()> {
        sqlx::query(
            "UPDATE d_places SET continent = ?, country = ?, city = ?, circa = ?, disputed = ? WHERE id = ?",
        )
        .bind(&place.continent)
        .bind(&place.country)
        .bind(&place.city)
        .bind(i64::from(place.circa))
        .bind(i64::from(place.disputed))
        .bind(place.id)
        .execute(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM d_places WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Place>> {
        let rows = sqlx::query(
            "SELECT id, continent, country, city, circa, disputed FROM d_places ORDER BY continent, country, city, id",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows.into_iter().map(Self::map_place).collect())
    }

    pub async fn search(&self, pattern: &str) -> RitmoResult<Vec<Place>> {
        let like_pattern = format!("%{pattern}%");
        let rows = sqlx::query(
            "SELECT id, continent, country, city, circa, disputed
             FROM d_places
             WHERE continent LIKE ? OR country LIKE ? OR city LIKE ?
             ORDER BY continent, country, city, id",
        )
        .bind(&like_pattern)
        .bind(&like_pattern)
        .bind(&like_pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows.into_iter().map(Self::map_place).collect())
    }

    fn map_place(row: sqlx::sqlite::SqliteRow) -> Place {
        Place {
            id: row.get("id"),
            continent: row.get("continent"),
            country: row.get("country"),
            city: row.get("city"),
            circa: row.get::<i64, _>("circa") != 0,
            disputed: row.get::<i64, _>("disputed") != 0,
        }
    }
}
