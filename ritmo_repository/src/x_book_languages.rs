use crate::support::{map_delete, map_insert, map_query, RepositoryContext};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct XBookLanguagesRepository {
    pool: SqlitePool,
}

impl XBookLanguagesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(
        &self,
        book_id: i64,
        language_id: i64,
        language_role_id: i64,
    ) -> RitmoResult<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO x_book_languages(book_id, language_id, role_id) VALUES (?, ?, ?)",
        )
        .bind(book_id)
        .bind(language_id)
        .bind(language_role_id)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(
        &self,
        book_id: i64,
        language_id: i64,
        language_role_id: i64,
    ) -> RitmoResult<()> {
        sqlx::query(
            "DELETE FROM x_book_languages WHERE book_id = ? AND language_id = ? AND role_id = ?",
        )
        .bind(book_id)
        .bind(language_id)
        .bind(language_role_id)
        .execute(&self.pool)
        .await
        .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_book(&self, book_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = sqlx::query("SELECT language_id, role_id FROM x_book_languages WHERE book_id = ?")
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("language_id"), row.get("role_id")))
            .collect())
    }

    /// Returns (language_id, official_name, role_id, role_label) for languages linked to a book.
    pub async fn list_with_roles_by_book(
        &self,
        book_id: i64,
    ) -> RitmoResult<Vec<(i64, String, i64, String)>> {
        let rows = sqlx::query(
            "SELECT l.id AS language_id, l.official_name,
                    r.id AS role_id,
                    COALESCE(rt.label, r.code) AS role_label
             FROM x_book_languages xbl
             INNER JOIN d_languages l ON l.id = xbl.language_id
             INNER JOIN s_book_language_roles r ON r.id = xbl.role_id
             LEFT JOIN s_book_language_role_translations rt
                    ON rt.role_id = r.id AND rt.language_code = 'it'
             WHERE xbl.book_id = ?
             ORDER BY r.code COLLATE NOCASE, l.official_name COLLATE NOCASE",
        )
        .bind(book_id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.get::<i64, _>("language_id"),
                    row.get::<String, _>("official_name"),
                    row.get::<i64, _>("role_id"),
                    row.get::<String, _>("role_label"),
                )
            })
            .collect())
    }

    pub async fn list_by_language(
        &self,
        language_id: i64,
        language_role_id: Option<i64>,
    ) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = match language_role_id {
            Some(role_id) => sqlx::query(
                "SELECT book_id, role_id FROM x_book_languages WHERE language_id = ? AND role_id = ?",
            )
            .bind(language_id)
            .bind(role_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?,
            None => {
                sqlx::query("SELECT book_id, role_id FROM x_book_languages WHERE language_id = ?")
                    .bind(language_id)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(map_query)?
            }
        };
        Ok(rows
            .into_iter()
            .map(|row| (row.get("book_id"), row.get("role_id")))
            .collect())
    }
}
