use crate::support::{map_delete, map_insert, map_query, not_found, RepositoryContext};
use ritmo_domain::Language;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct LanguageRepository {
    pool: SqlitePool,
}

impl LanguageRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, language: &Language) -> RitmoResult<i64> {
        let result = sqlx::query(
            "INSERT INTO languages(iso_code_2char, iso_code_3char, official_name) VALUES (?, ?, ?)",
        )
        .bind(&language.iso_639_2)
        .bind(&language.iso_639_3)
        .bind(&language.name)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Language> {
        let row = sqlx::query(
            "SELECT id, iso_code_2char, iso_code_3char, official_name FROM languages WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?
        .ok_or_else(not_found)?;

        Ok(Language {
            id: row.get("id"),
            iso_639_2: row.get("iso_code_2char"),
            iso_639_3: row.get("iso_code_3char"),
            name: row.get("official_name"),
        })
    }

    pub async fn update(&self, language: &Language) -> RitmoResult<()> {
        sqlx::query(
            "UPDATE languages SET iso_code_2char = ?, iso_code_3char = ?, official_name = ? WHERE id = ?",
        )
        .bind(&language.iso_639_2)
        .bind(&language.iso_639_3)
        .bind(&language.name)
        .bind(language.id)
        .execute(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM languages WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Language>> {
        let rows = sqlx::query(
            "SELECT id, iso_code_2char, iso_code_3char, official_name FROM languages ORDER BY official_name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| Language {
                id: row.get("id"),
                iso_639_2: row.get("iso_code_2char"),
                iso_639_3: row.get("iso_code_3char"),
                name: row.get("official_name"),
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Language>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query(
            "SELECT id, iso_code_2char, iso_code_3char, official_name FROM languages WHERE official_name LIKE ? COLLATE NOCASE ORDER BY official_name",
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| Language {
                id: row.get("id"),
                iso_639_2: row.get("iso_code_2char"),
                iso_639_3: row.get("iso_code_3char"),
                name: row.get("official_name"),
            })
            .collect())
    }

    pub async fn get_or_create(&self, name: &str) -> RitmoResult<Language> {
        if let Some(row) = sqlx::query(
            "SELECT id, iso_code_2char, iso_code_3char, official_name FROM languages WHERE official_name = ?",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?
        {
            return Ok(Language {
                id: row.get("id"),
                iso_639_2: row.get("iso_code_2char"),
                iso_639_3: row.get("iso_code_3char"),
                name: row.get("official_name"),
            });
        }

        let created = Language {
            id: 0,
            iso_639_2: None,
            iso_639_3: None,
            name: name.to_string(),
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
