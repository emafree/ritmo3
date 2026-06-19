use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::Publisher;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct PublisherRepository {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct PublisherDetailData {
    pub id: i64,
    pub name: String,
    pub country: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub places: Vec<(
        i64,
        Option<String>,
        Option<String>,
        Option<String>,
        bool,
        bool,
        String,
        String,
    )>,
}

impl PublisherRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, item: &Publisher) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT OR IGNORE INTO d_publishers(name) VALUES (?)")
            .bind(&item.name)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Publisher> {
        let row = sqlx::query("SELECT id, name FROM d_publishers WHERE id = ?")
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

    pub async fn get_detail(&self, id: i64) -> RitmoResult<PublisherDetailData> {
        let rows = sqlx::query(
            "SELECT p.id, p.name, p.country, p.website, p.notes,
                    pl.id AS place_id, pl.continent, pl.country AS place_country, pl.city,
                    pl.circa, pl.disputed, pt.key, ptt.label
             FROM d_publishers p
             LEFT JOIN x_publisher_places xpp ON xpp.publisher_id = p.id
             LEFT JOIN d_places pl ON pl.id = xpp.place_id
             LEFT JOIN s_place_types pt ON pt.id = xpp.place_type_id
             LEFT JOIN s_place_type_translations ptt ON ptt.place_type_id = pt.id AND ptt.language_code = 'it'
             WHERE p.id = ?",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        if rows.is_empty() {
            return Err(not_found());
        }

        let first = &rows[0];
        let publisher_id: i64 = first.get("id");
        let name: String = first.get("name");
        let country: Option<String> = first.get("country");
        let website: Option<String> = first.get("website");
        let notes: Option<String> = first.get("notes");
        let places = rows
            .into_iter()
            .filter_map(|row| {
                row.get::<Option<i64>, _>("place_id").map(|place_id| {
                    let place_type_key = row
                        .get::<Option<String>, _>("key")
                        .unwrap_or_else(|| "other".to_owned());
                    let place_type_label = row
                        .get::<Option<String>, _>("label")
                        .unwrap_or_else(|| place_type_key.clone());

                    (
                        place_id,
                        row.get::<Option<String>, _>("continent"),
                        row.get::<Option<String>, _>("place_country"),
                        row.get::<Option<String>, _>("city"),
                        row.get::<Option<i64>, _>("circa").unwrap_or(0) != 0,
                        row.get::<Option<i64>, _>("disputed").unwrap_or(0) != 0,
                        place_type_key,
                        place_type_label,
                    )
                })
            })
            .collect();

        Ok(PublisherDetailData {
            id: publisher_id,
            name,
            country,
            website,
            notes,
            places,
        })
    }

    pub async fn update(&self, item: &Publisher) -> RitmoResult<()> {
        sqlx::query("UPDATE d_publishers SET name = ? WHERE id = ?")
            .bind(&item.name)
            .bind(item.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn is_referenced(&self, id: i64) -> RitmoResult<i64> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM d_books WHERE publisher_id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(map_query)
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM d_publishers WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Publisher>> {
        let rows = sqlx::query("SELECT id, name FROM d_publishers ORDER BY name")
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
            "SELECT id, name FROM d_publishers WHERE name LIKE ? COLLATE NOCASE ORDER BY name",
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
        let value = value.trim();

        if let Some(row) = sqlx::query("SELECT id, name FROM d_publishers WHERE TRIM(name) = ? COLLATE NOCASE")
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
        if id > 0 {
            return self.get(id).await;
        }

        let row = sqlx::query("SELECT id, name FROM d_publishers WHERE TRIM(name) = ? COLLATE NOCASE")
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(Publisher {
            id: row.get("id"),
            name: row.get("name"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RepositoryContext;

    #[tokio::test]
    async fn get_detail_returns_publisher_with_places() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let ctx = RepositoryContext::new(pool);
        let repo = PublisherRepository::new(&ctx);
        let publisher_id = repo
            .save(&Publisher {
                id: 0,
                name: "Editore".to_owned(),
            })
            .await
            .unwrap();

        sqlx::query("UPDATE d_publishers SET country = ?, website = ?, notes = ? WHERE id = ?")
            .bind("Italia")
            .bind("https://example.org")
            .bind("Note")
            .bind(publisher_id)
            .execute(ctx.pool())
            .await
            .unwrap();

        let place_id =
            sqlx::query("INSERT INTO d_places(continent, country, city) VALUES (?, ?, ?)")
                .bind("Europa")
                .bind("Italia")
                .bind("Milano")
                .execute(ctx.pool())
                .await
                .unwrap()
                .last_insert_rowid();
        let place_type_id =
            sqlx::query_scalar::<_, i64>("SELECT id FROM s_place_types WHERE key = ?")
                .bind("activity")
                .fetch_one(ctx.pool())
                .await
                .unwrap();
        sqlx::query(
            "INSERT INTO x_publisher_places(publisher_id, place_id, place_type_id) VALUES (?, ?, ?)",
        )
        .bind(publisher_id)
        .bind(place_id)
        .bind(place_type_id)
        .execute(ctx.pool())
        .await
        .unwrap();

        let detail = repo.get_detail(publisher_id).await.unwrap();
        assert_eq!(detail.name, "Editore");
        assert_eq!(detail.country.as_deref(), Some("Italia"));
        assert_eq!(detail.website.as_deref(), Some("https://example.org"));
        assert_eq!(detail.notes.as_deref(), Some("Note"));
        assert_eq!(detail.places.len(), 1);
        assert_eq!(detail.places[0].0, place_id);
        assert_eq!(detail.places[0].6, "activity");
        assert!(!detail.places[0].7.is_empty());
    }

    #[tokio::test]
    async fn get_or_create_reuses_existing_publisher_case_insensitively() {
        let pool = ritmo_db::create_sqlite_pool("sqlite::memory:")
            .await
            .unwrap();
        let ctx = RepositoryContext::new(pool);
        let repo = PublisherRepository::new(&ctx);
        let existing_id = repo
            .save(&Publisher {
                id: 0,
                name: "Einaudi".to_owned(),
            })
            .await
            .unwrap();

        let publisher = repo.get_or_create("  einaudi  ").await.unwrap();

        assert_eq!(publisher.id, existing_id);
        assert_eq!(publisher.name, "Einaudi");
    }
}
