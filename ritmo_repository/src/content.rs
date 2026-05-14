use crate::support::{
    map_delete, map_insert, map_query, not_found, partial_date_from_parts, partial_date_to_parts,
    RepositoryContext,
};
use ritmo_domain::Content;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct ContentRepository {
    pool: SqlitePool,
}

impl ContentRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, content: &Content) -> RitmoResult<i64> {
        let (year, month, day, circa) = partial_date_to_parts(&content.publication_year);
        let result = sqlx::query(
            "INSERT OR IGNORE INTO contents(name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&content.title)
        .bind(year)
        .bind(month)
        .bind(day)
        .bind(circa)
        .bind(&content.notes)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Content> {
        let row = sqlx::query("SELECT id, name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM contents WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;

        Ok(Content {
            id: row.get("id"),
            title: row.get("name"),
            publication_year: partial_date_from_parts(
                row.get("publication_date_year"),
                row.get("publication_date_month"),
                row.get("publication_date_day"),
                row.get::<i64, _>("publication_date_circa"),
            ),
            notes: row.get("notes"),
        })
    }

    pub async fn update(&self, content: &Content) -> RitmoResult<()> {
        let (year, month, day, circa) = partial_date_to_parts(&content.publication_year);
        sqlx::query(
            "UPDATE contents SET name = ?, publication_date_year = ?, publication_date_month = ?, publication_date_day = ?, publication_date_circa = ?, notes = ? WHERE id = ?",
        )
        .bind(&content.title)
        .bind(year)
        .bind(month)
        .bind(day)
        .bind(circa)
        .bind(&content.notes)
        .bind(content.id)
        .execute(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM contents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Content>> {
        let rows = sqlx::query("SELECT id, name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM contents ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Content {
                id: row.get("id"),
                title: row.get("name"),
                publication_year: partial_date_from_parts(
                    row.get("publication_date_year"),
                    row.get("publication_date_month"),
                    row.get("publication_date_day"),
                    row.get::<i64, _>("publication_date_circa"),
                ),
                notes: row.get("notes"),
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Content>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query("SELECT id, name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM contents WHERE name LIKE ? COLLATE NOCASE ORDER BY name")
            .bind(pattern)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Content {
                id: row.get("id"),
                title: row.get("name"),
                publication_year: partial_date_from_parts(
                    row.get("publication_date_year"),
                    row.get("publication_date_month"),
                    row.get("publication_date_day"),
                    row.get::<i64, _>("publication_date_circa"),
                ),
                notes: row.get("notes"),
            })
            .collect())
    }

    pub async fn get_or_create(&self, title: &str) -> RitmoResult<Content> {
        if let Some(row) = sqlx::query("SELECT id, name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM contents WHERE name = ?")
            .bind(title)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(Content {
                id: row.get("id"),
                title: row.get("name"),
                publication_year: partial_date_from_parts(
                    row.get("publication_date_year"),
                    row.get("publication_date_month"),
                    row.get("publication_date_day"),
                    row.get::<i64, _>("publication_date_circa"),
                ),
                notes: row.get("notes"),
            });
        }

        let created = Content {
            id: 0,
            title: title.to_string(),
            publication_year: None,
            notes: None,
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
