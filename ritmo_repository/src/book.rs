use crate::support::{
    map_delete, map_insert, map_query, not_found, partial_date_from_parts, partial_date_to_parts,
    RepositoryContext,
};
use ritmo_domain::Book;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct BookRepository {
    pool: SqlitePool,
}

impl BookRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, book: &Book) -> RitmoResult<i64> {
        let (year, month, day, circa) = partial_date_to_parts(&book.publication_year);
        let result = sqlx::query(
            "INSERT OR IGNORE INTO books(name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&book.title)
        .bind(&book.isbn)
        .bind(year)
        .bind(month)
        .bind(day)
        .bind(circa)
        .bind(&book.notes)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Book> {
        let row = sqlx::query("SELECT id, name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM books WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;

        Ok(Book {
            id: row.get("id"),
            title: row.get("name"),
            isbn: row.get("isbn"),
            publication_year: partial_date_from_parts(
                row.get("publication_date_year"),
                row.get("publication_date_month"),
                row.get("publication_date_day"),
                row.get::<i64, _>("publication_date_circa"),
            ),
            notes: row.get("notes"),
        })
    }

    pub async fn update(&self, book: &Book) -> RitmoResult<()> {
        let (year, month, day, circa) = partial_date_to_parts(&book.publication_year);
        sqlx::query(
            "UPDATE books SET name = ?, isbn = ?, publication_date_year = ?, publication_date_month = ?, publication_date_day = ?, publication_date_circa = ?, notes = ? WHERE id = ?",
        )
        .bind(&book.title)
        .bind(&book.isbn)
        .bind(year)
        .bind(month)
        .bind(day)
        .bind(circa)
        .bind(&book.notes)
        .bind(book.id)
        .execute(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM books WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Book>> {
        let rows = sqlx::query("SELECT id, name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM books ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Book {
                id: row.get("id"),
                title: row.get("name"),
                isbn: row.get("isbn"),
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

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Book>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query("SELECT id, name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM books WHERE name LIKE ? COLLATE NOCASE ORDER BY name")
            .bind(pattern)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Book {
                id: row.get("id"),
                title: row.get("name"),
                isbn: row.get("isbn"),
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

    pub async fn get_format_name(&self, book_id: i64) -> RitmoResult<Option<String>> {
        let row = sqlx::query(
            "SELECT f.key FROM books b INNER JOIN formats f ON b.format_id = f.id WHERE b.id = ?",
        )
        .bind(book_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(row.map(|r| r.get::<String, _>("key")))
    }

    pub async fn get_series_name(&self, book_id: i64) -> RitmoResult<Option<String>> {
        let row = sqlx::query(
            "SELECT s.name FROM books b INNER JOIN series s ON b.series_id = s.id WHERE b.id = ?",
        )
        .bind(book_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(row.map(|r| r.get::<String, _>("name")))
    }

    pub async fn get_or_create(&self, title: &str) -> RitmoResult<Book> {
        if let Some(row) = sqlx::query("SELECT id, name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM books WHERE name = ?")
            .bind(title)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(Book {
                id: row.get("id"),
                title: row.get("name"),
                isbn: row.get("isbn"),
                publication_year: partial_date_from_parts(
                    row.get("publication_date_year"),
                    row.get("publication_date_month"),
                    row.get("publication_date_day"),
                    row.get::<i64, _>("publication_date_circa"),
                ),
                notes: row.get("notes"),
            });
        }

        let created = Book {
            id: 0,
            title: title.to_string(),
            isbn: None,
            publication_year: None,
            notes: None,
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
