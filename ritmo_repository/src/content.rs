use crate::support::{
    map_delete, map_insert, map_query, not_found, partial_date_from_parts, partial_date_to_parts,
    RepositoryContext,
};
use ritmo_domain::{Content, PartialDate};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct ContentRepository {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct ContentDetailData {
    pub id: i64,
    pub name: String,
    pub original_title: Option<String>,
    pub type_id: Option<i64>,
    pub content_type: Option<String>,
    pub publication_date: Option<PartialDate>,
    pub notes: Option<String>,
    pub books: Vec<(i64, String)>,
    pub people: Vec<(i64, String, String)>,
    pub tags: Vec<(String, String)>,
    pub languages: Vec<(String, Option<String>, String)>,
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
            "INSERT OR IGNORE INTO d_contents(name, original_title, type_id, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&content.title)
        .bind(&content.original_title)
        .bind(content.type_id)
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
        let row = sqlx::query("SELECT id, name, original_title, type_id, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_contents WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;

        Ok(Content {
            id: row.get("id"),
            title: row.get("name"),
            original_title: row.get("original_title"),
            type_id: row.get("type_id"),
            publication_year: partial_date_from_parts(
                row.get("publication_date_year"),
                row.get("publication_date_month"),
                row.get("publication_date_day"),
                row.get::<i64, _>("publication_date_circa"),
            ),
            notes: row.get("notes"),
        })
    }

    pub async fn get_detail(&self, id: i64) -> RitmoResult<ContentDetailData> {
        let row = sqlx::query(
            "SELECT
                c.id,
                c.name,
                c.original_title,
                c.type_id,
                t.key AS type_key,
                c.publication_date_year,
                c.publication_date_month,
                c.publication_date_day,
                c.publication_date_circa,
                c.notes
             FROM d_contents c
             LEFT JOIN d_types t ON t.id = c.type_id
             WHERE c.id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?
        .ok_or_else(not_found)?;

        let books = sqlx::query(
            "SELECT b.id, b.name
             FROM x_books_contents xbc
             INNER JOIN d_books b ON b.id = xbc.book_id
             WHERE xbc.content_id = ?
             ORDER BY b.name COLLATE NOCASE",
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
             FROM x_contents_people_roles xcpr
             INNER JOIN d_people p ON p.id = xcpr.person_id
             INNER JOIN d_roles r ON r.id = xcpr.role_id
             WHERE xcpr.content_id = ?
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
             FROM x_contents_tags xct
             INNER JOIN d_tags t ON t.id = xct.tag_id
             WHERE xct.content_id = ?
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
             FROM x_content_languages xcl
             INNER JOIN d_languages l ON l.id = xcl.language_id
             INNER JOIN s_content_language_roles lr ON lr.id = xcl.role_id
             WHERE xcl.content_id = ?
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

        Ok(ContentDetailData {
            id: row.get("id"),
            name: row.get("name"),
            original_title: row.get("original_title"),
            type_id: row.get("type_id"),
            content_type: row.get("type_key"),
            publication_date: partial_date_from_parts(
                row.get("publication_date_year"),
                row.get("publication_date_month"),
                row.get("publication_date_day"),
                row.get::<i64, _>("publication_date_circa"),
            ),
            notes: row.get("notes"),
            books,
            people,
            tags,
            languages,
        })
    }

    pub async fn update(&self, content: &Content) -> RitmoResult<()> {
        let (year, month, day, circa) = partial_date_to_parts(&content.publication_year);
        sqlx::query(
            "UPDATE d_contents SET name = ?, original_title = ?, type_id = ?, publication_date_year = ?, publication_date_month = ?, publication_date_day = ?, publication_date_circa = ?, notes = ? WHERE id = ?",
        )
        .bind(&content.title)
        .bind(&content.original_title)
        .bind(content.type_id)
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
        sqlx::query("DELETE FROM d_contents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Content>> {
        let rows = sqlx::query("SELECT id, name, original_title, type_id, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_contents ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Content {
                id: row.get("id"),
                title: row.get("name"),
                original_title: row.get("original_title"),
                type_id: row.get("type_id"),
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
        let rows = sqlx::query("SELECT id, name, original_title, type_id, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_contents WHERE name LIKE ? COLLATE NOCASE ORDER BY name")
            .bind(pattern)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Content {
                id: row.get("id"),
                title: row.get("name"),
                original_title: row.get("original_title"),
                type_id: row.get("type_id"),
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

    pub async fn list_all_with_people(
        pool: &SqlitePool,
    ) -> RitmoResult<Vec<(i64, String, Option<String>, Option<String>, Vec<String>)>> {
        let rows = sqlx::query(
            "SELECT
                c.id,
                c.name,
                t.key AS type_key,
                (
                    SELECT l.native_name
                    FROM x_content_languages xcl
                    JOIN d_languages l ON l.id = xcl.language_id
                    JOIN s_content_language_roles clr ON clr.id = xcl.role_id
                    WHERE xcl.content_id = c.id
                      AND clr.code = 'original'
                    LIMIT 1
                ) AS original_language,
                GROUP_CONCAT(
                    CASE WHEN r.key = 'author' THEN p.name ELSE NULL END,
                    '||'
                ) AS authors_concat
            FROM d_contents c
            LEFT JOIN d_types t ON t.id = c.type_id
            LEFT JOIN x_contents_people_roles cpr ON cpr.content_id = c.id
            LEFT JOIN d_roles r ON r.id = cpr.role_id
            LEFT JOIN d_people p ON p.id = cpr.person_id
            GROUP BY c.id
            ORDER BY c.name COLLATE NOCASE",
        )
        .fetch_all(pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let authors_concat: Option<String> = row.get("authors_concat");
                let authors = authors_concat
                    .map(|value| {
                        value
                            .split("||")
                            .filter(|name| !name.is_empty())
                            .map(str::to_owned)
                            .collect()
                    })
                    .unwrap_or_default();

                (
                    row.get::<i64, _>("id"),
                    row.get::<String, _>("name"),
                    row.get::<Option<String>, _>("type_key"),
                    row.get::<Option<String>, _>("original_language"),
                    authors,
                )
            })
            .collect())
    }

    pub async fn get_or_create(&self, title: &str) -> RitmoResult<Content> {
        if let Some(row) = sqlx::query("SELECT id, name, original_title, type_id, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_contents WHERE name = ?")
            .bind(title)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
        {
            return Ok(Content {
                id: row.get("id"),
                title: row.get("name"),
                original_title: row.get("original_title"),
                type_id: row.get("type_id"),
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
            original_title: None,
            type_id: None,
            publication_year: None,
            notes: None,
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
