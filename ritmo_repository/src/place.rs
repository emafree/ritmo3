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

    pub async fn save(
        &self,
        continent: Option<String>,
        country: Option<String>,
        city: Option<String>,
        circa: bool,
        disputed: bool,
    ) -> RitmoResult<i64> {
        let result = sqlx::query(
            "INSERT OR IGNORE INTO d_places(continent, country, city, circa, disputed) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(continent)
        .bind(country)
        .bind(city)
        .bind(i64::from(circa))
        .bind(i64::from(disputed))
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

    pub async fn update(
        &self,
        id: i64,
        continent: Option<String>,
        country: Option<String>,
        city: Option<String>,
        circa: bool,
        disputed: bool,
    ) -> RitmoResult<()> {
        sqlx::query(
            "UPDATE d_places SET continent = ?, country = ?, city = ?, circa = ?, disputed = ? WHERE id = ?",
        )
        .bind(continent)
        .bind(country)
        .bind(city)
        .bind(i64::from(circa))
        .bind(i64::from(disputed))
        .bind(id)
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
             WHERE country LIKE ? COLLATE NOCASE OR city LIKE ? COLLATE NOCASE
             ORDER BY continent, country, city, id",
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RepositoryContext;

    #[tokio::test]
    async fn save_get_update_delete_place() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let ctx = RepositoryContext::new(pool);
        let repo = PlaceRepository::new(&ctx);

        let place_id = repo
            .save(
                Some("Europa".to_owned()),
                Some("Italia".to_owned()),
                Some("Roma".to_owned()),
                false,
                false,
            )
            .await
            .unwrap();

        let place = repo.get(place_id).await.unwrap();
        assert_eq!(place.city.as_deref(), Some("Roma"));

        repo.update(
            place_id,
            Some("Europa".to_owned()),
            Some("Italia".to_owned()),
            Some("Milano".to_owned()),
            true,
            false,
        )
        .await
        .unwrap();

        let updated = repo.get(place_id).await.unwrap();
        assert_eq!(updated.city.as_deref(), Some("Milano"));
        assert!(updated.circa);

        repo.delete(place_id).await.unwrap();
        assert!(repo.get(place_id).await.is_err());
    }

    #[tokio::test]
    async fn search_matches_country_and_city_case_insensitive() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let ctx = RepositoryContext::new(pool);
        let repo = PlaceRepository::new(&ctx);

        repo.save(
            Some("Europa".to_owned()),
            Some("Italia".to_owned()),
            Some("Roma".to_owned()),
            false,
            false,
        )
        .await
        .unwrap();
        repo.save(
            Some("Europa".to_owned()),
            Some("Francia".to_owned()),
            Some("Parigi".to_owned()),
            false,
            false,
        )
        .await
        .unwrap();

        let by_country = repo.search("itaLIA").await.unwrap();
        assert_eq!(by_country.len(), 1);
        assert_eq!(by_country[0].country.as_deref(), Some("Italia"));

        let by_city = repo.search("parI").await.unwrap();
        assert_eq!(by_city.len(), 1);
        assert_eq!(by_city[0].city.as_deref(), Some("Parigi"));
    }
}
