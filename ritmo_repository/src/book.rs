use crate::support::{
    map_delete, map_insert, map_query, not_found, partial_date_from_parts, partial_date_to_parts,
    RepositoryContext,
};
use ritmo_domain::{Book, PartialDate};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct BookRepository {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct BookDetailData {
    pub id: i64,
    pub name: String,
    pub original_title: Option<String>,
    pub publisher_id: Option<i64>,
    pub publisher: Option<String>,
    pub format_id: Option<i64>,
    pub format: Option<String>,
    pub series_id: Option<i64>,
    pub series: Option<String>,
    pub series_index: Option<i64>,
    pub publication_date: Option<PartialDate>,
    pub isbn: Option<String>,
    pub notes: Option<String>,
    pub has_cover: bool,
    pub has_paper: bool,
    pub contents: Vec<(i64, String)>,
    pub people: Vec<(i64, String, String)>,
    pub tags: Vec<(String, String)>,
    pub languages: Vec<(String, Option<String>, String)>,
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
            "INSERT OR IGNORE INTO d_books(name, original_title, publisher_id, format_id, series_id, series_index, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes, has_cover, has_paper) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&book.title)
        .bind(&book.original_title)
        .bind(book.publisher_id)
        .bind(book.format_id)
        .bind(book.series_id)
        .bind(book.series_index)
        .bind(&book.isbn)
        .bind(year)
        .bind(month)
        .bind(day)
        .bind(circa)
        .bind(&book.notes)
        .bind(i64::from(book.has_cover))
        .bind(i64::from(book.has_paper))
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Book> {
        let row = sqlx::query("SELECT id, name, original_title, publisher_id, format_id, series_id, series_index, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes, has_cover, has_paper FROM d_books WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;

        Ok(Book {
            id: row.get("id"),
            title: row.get("name"),
            original_title: row.get("original_title"),
            publisher_id: row.get("publisher_id"),
            format_id: row.get("format_id"),
            series_id: row.get("series_id"),
            series_index: row.get("series_index"),
            isbn: row.get("isbn"),
            publication_year: partial_date_from_parts(
                row.get("publication_date_year"),
                row.get("publication_date_month"),
                row.get("publication_date_day"),
                row.get::<i64, _>("publication_date_circa"),
            ),
            notes: row.get("notes"),
            has_cover: row.get::<i64, _>("has_cover") != 0,
            has_paper: row.get::<i64, _>("has_paper") != 0,
        })
    }

    pub async fn get_detail(&self, id: i64) -> RitmoResult<BookDetailData> {
        let row = sqlx::query(
            "SELECT
                b.id,
                b.name,
                b.original_title,
                b.publisher_id,
                p.name AS publisher_name,
                b.format_id,
                f.key AS format_key,
                b.series_id,
                s.name AS series_name,
                b.series_index,
                b.publication_date_year,
                b.publication_date_month,
                b.publication_date_day,
                b.publication_date_circa,
                b.isbn,
                b.notes,
                b.has_cover,
                b.has_paper
             FROM d_books b
             LEFT JOIN d_publishers p ON p.id = b.publisher_id
             LEFT JOIN d_formats f ON f.id = b.format_id
             LEFT JOIN d_series s ON s.id = b.series_id
             WHERE b.id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?
        .ok_or_else(not_found)?;

        let contents = sqlx::query(
            "SELECT c.id, c.name
             FROM x_books_contents xbc
             INNER JOIN d_contents c ON c.id = xbc.content_id
             WHERE xbc.book_id = ?
             ORDER BY c.name COLLATE NOCASE",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?
        .into_iter()
        .map(|row| (row.get::<i64, _>("id"), row.get::<String, _>("name")))
        .collect();

        let people = sqlx::query(
            "SELECT p.id, p.name, r.key AS role_key
             FROM x_books_people_roles xbpr
             INNER JOIN d_people p ON p.id = xbpr.person_id
             INNER JOIN d_roles r ON r.id = xbpr.role_id
             WHERE xbpr.book_id = ?
             ORDER BY r.key COLLATE NOCASE, p.name COLLATE NOCASE",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?
        .into_iter()
        .map(|row| {
            (
                row.get::<i64, _>("id"),
                row.get::<String, _>("name"),
                row.get::<String, _>("role_key"),
            )
        })
        .collect();

        let tags = sqlx::query(
            "SELECT t.name, t.tag_type
             FROM x_books_tags xbt
             INNER JOIN d_tags t ON t.id = xbt.tag_id
             WHERE xbt.book_id = ?
             ORDER BY t.tag_type COLLATE NOCASE, t.name COLLATE NOCASE",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?
        .into_iter()
        .map(|row| {
            (
                row.get::<String, _>("name"),
                row.get::<String, _>("tag_type"),
            )
        })
        .collect();

        let languages = sqlx::query(
            "SELECT l.official_name, l.iso_code_2char, lr.code AS role_code
             FROM x_book_languages xbl
             INNER JOIN d_languages l ON l.id = xbl.language_id
             INNER JOIN s_book_language_roles lr ON lr.id = xbl.role_id
             WHERE xbl.book_id = ?
             ORDER BY lr.code COLLATE NOCASE, l.official_name COLLATE NOCASE",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?
        .into_iter()
        .map(|row| {
            (
                row.get::<String, _>("official_name"),
                row.get::<Option<String>, _>("iso_code_2char"),
                row.get::<String, _>("role_code"),
            )
        })
        .collect();

        Ok(BookDetailData {
            id: row.get("id"),
            name: row.get("name"),
            original_title: row.get("original_title"),
            publisher_id: row.get("publisher_id"),
            publisher: row.get("publisher_name"),
            format_id: row.get("format_id"),
            format: row.get("format_key"),
            series_id: row.get("series_id"),
            series: row.get("series_name"),
            series_index: row.get("series_index"),
            publication_date: partial_date_from_parts(
                row.get("publication_date_year"),
                row.get("publication_date_month"),
                row.get("publication_date_day"),
                row.get::<i64, _>("publication_date_circa"),
            ),
            isbn: row.get("isbn"),
            notes: row.get("notes"),
            has_cover: row.get::<i64, _>("has_cover") != 0,
            has_paper: row.get::<i64, _>("has_paper") != 0,
            contents,
            people,
            tags,
            languages,
        })
    }

    pub async fn update(&self, book: &Book) -> RitmoResult<()> {
        let (year, month, day, circa) = partial_date_to_parts(&book.publication_year);
        sqlx::query(
            "UPDATE d_books SET name = ?, original_title = ?, publisher_id = ?, format_id = ?, series_id = ?, series_index = ?, isbn = ?, publication_date_year = ?, publication_date_month = ?, publication_date_day = ?, publication_date_circa = ?, notes = ?, has_cover = ?, has_paper = ? WHERE id = ?",
        )
        .bind(&book.title)
        .bind(&book.original_title)
        .bind(book.publisher_id)
        .bind(book.format_id)
        .bind(book.series_id)
        .bind(book.series_index)
        .bind(&book.isbn)
        .bind(year)
        .bind(month)
        .bind(day)
        .bind(circa)
        .bind(&book.notes)
        .bind(i64::from(book.has_cover))
        .bind(i64::from(book.has_paper))
        .bind(book.id)
        .execute(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM d_books WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Book>> {
        let rows = sqlx::query("SELECT id, name, original_title, publisher_id, format_id, series_id, series_index, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes, has_cover, has_paper FROM d_books ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Book {
                id: row.get("id"),
                title: row.get("name"),
                original_title: row.get("original_title"),
                publisher_id: row.get("publisher_id"),
                format_id: row.get("format_id"),
                series_id: row.get("series_id"),
                series_index: row.get("series_index"),
                isbn: row.get("isbn"),
                publication_year: partial_date_from_parts(
                    row.get("publication_date_year"),
                    row.get("publication_date_month"),
                    row.get("publication_date_day"),
                    row.get::<i64, _>("publication_date_circa"),
                ),
                notes: row.get("notes"),
                has_cover: row.get::<i64, _>("has_cover") != 0,
                has_paper: row.get::<i64, _>("has_paper") != 0,
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Book>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query("SELECT id, name, original_title, publisher_id, format_id, series_id, series_index, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes, has_cover, has_paper FROM d_books WHERE name LIKE ? COLLATE NOCASE ORDER BY name")
            .bind(pattern)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Book {
                id: row.get("id"),
                title: row.get("name"),
                original_title: row.get("original_title"),
                publisher_id: row.get("publisher_id"),
                format_id: row.get("format_id"),
                series_id: row.get("series_id"),
                series_index: row.get("series_index"),
                isbn: row.get("isbn"),
                publication_year: partial_date_from_parts(
                    row.get("publication_date_year"),
                    row.get("publication_date_month"),
                    row.get("publication_date_day"),
                    row.get::<i64, _>("publication_date_circa"),
                ),
                notes: row.get("notes"),
                has_cover: row.get::<i64, _>("has_cover") != 0,
                has_paper: row.get::<i64, _>("has_paper") != 0,
            })
            .collect())
    }

    pub async fn get_format_name(&self, book_id: i64) -> RitmoResult<Option<String>> {
        let row = sqlx::query(
            "SELECT f.key FROM d_books b INNER JOIN d_formats f ON b.format_id = f.id WHERE b.id = ?",
        )
        .bind(book_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(row.map(|r| r.get::<String, _>("key")))
    }

    pub async fn get_series_name(&self, book_id: i64) -> RitmoResult<Option<String>> {
        let row = sqlx::query(
            "SELECT s.name FROM d_books b INNER JOIN d_series s ON b.series_id = s.id WHERE b.id = ?",
        )
        .bind(book_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(row.map(|r| r.get::<String, _>("name")))
    }

    pub async fn get_or_create(&self, title: &str) -> RitmoResult<Book> {
        if let Some(row) = sqlx::query("SELECT id, name, original_title, publisher_id, format_id, series_id, series_index, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes, has_cover, has_paper FROM d_books WHERE name = ?")
            .bind(title)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(Book {
                id: row.get("id"),
                title: row.get("name"),
                original_title: row.get("original_title"),
                publisher_id: row.get("publisher_id"),
                format_id: row.get("format_id"),
                series_id: row.get("series_id"),
                series_index: row.get("series_index"),
                isbn: row.get("isbn"),
                publication_year: partial_date_from_parts(
                    row.get("publication_date_year"),
                    row.get("publication_date_month"),
                    row.get("publication_date_day"),
                    row.get::<i64, _>("publication_date_circa"),
                ),
                notes: row.get("notes"),
                has_cover: row.get::<i64, _>("has_cover") != 0,
                has_paper: row.get::<i64, _>("has_paper") != 0,
            });
        }

        let created = Book {
            id: 0,
            title: title.to_string(),
            original_title: None,
            publisher_id: None,
            format_id: None,
            series_id: None,
            series_index: None,
            isbn: None,
            publication_year: None,
            notes: None,
            has_cover: false,
            has_paper: false,
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }

    pub async fn list_all_with_authors(
        &self,
    ) -> RitmoResult<Vec<(i64, String, Vec<String>, Option<String>, Option<String>)>> {
        // (book_id, title, authors, format_key, series_name)
        let rows = sqlx::query(
            "SELECT b.id, b.name,
                    GROUP_CONCAT(p.name, '||') AS authors,
                    f.key AS format_key,
                    s.name AS series_name
             FROM d_books b
             LEFT JOIN x_books_people_roles xbpr ON b.id = xbpr.book_id
             LEFT JOIN d_roles r ON xbpr.role_id = r.id AND r.key = 'author'
             LEFT JOIN d_people p ON xbpr.person_id = p.id AND r.key = 'author'
             LEFT JOIN d_formats f ON b.format_id = f.id
             LEFT JOIN d_series s ON b.series_id = s.id
             GROUP BY b.id
             ORDER BY b.name COLLATE NOCASE",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let authors_raw: Option<String> = row.get("authors");
                let authors = authors_raw
                    .map(|s| s.split("||").map(str::to_owned).collect())
                    .unwrap_or_default();
                (
                    row.get::<i64, _>("id"),
                    row.get::<String, _>("name"),
                    authors,
                    row.get::<Option<String>, _>("format_key"),
                    row.get::<Option<String>, _>("series_name"),
                )
            })
            .collect())
    }
}
