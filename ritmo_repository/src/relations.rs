use crate::support::{map_delete, map_insert, map_query, RepositoryContext};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct XBooksContentsRepository {
    pool: SqlitePool,
}

impl XBooksContentsRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, book_id: i64, content_id: i64) -> RitmoResult<()> {
        sqlx::query("INSERT INTO x_books_contents(book_id, content_id) VALUES (?, ?)")
            .bind(book_id)
            .bind(content_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, book_id: i64, content_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_books_contents WHERE book_id = ? AND content_id = ?")
            .bind(book_id)
            .bind(content_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_book(&self, book_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT content_id FROM x_books_contents WHERE book_id = ?")
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("content_id")).collect())
    }

    pub async fn list_by_content(&self, content_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT book_id FROM x_books_contents WHERE content_id = ?")
            .bind(content_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("book_id")).collect())
    }
}

pub struct XBooksPeopleRolesRepository {
    pool: SqlitePool,
}

impl XBooksPeopleRolesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, book_id: i64, person_id: i64, role_id: i64) -> RitmoResult<()> {
        sqlx::query(
            "INSERT INTO x_books_people_roles(book_id, person_id, role_id) VALUES (?, ?, ?)",
        )
        .bind(book_id)
        .bind(person_id)
        .bind(role_id)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, book_id: i64, person_id: i64, role_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_books_people_roles WHERE book_id = ? AND person_id = ? AND role_id = ?")
            .bind(book_id)
            .bind(person_id)
            .bind(role_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_book(&self, book_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = sqlx::query("SELECT person_id, role_id FROM x_books_people_roles WHERE book_id = ?")
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("person_id"), row.get("role_id")))
            .collect())
    }

    pub async fn list_by_person(&self, person_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = sqlx::query("SELECT book_id, role_id FROM x_books_people_roles WHERE person_id = ?")
            .bind(person_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("book_id"), row.get("role_id")))
            .collect())
    }

    pub async fn list_by_role(&self, role_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = sqlx::query("SELECT book_id, person_id FROM x_books_people_roles WHERE role_id = ?")
            .bind(role_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("book_id"), row.get("person_id")))
            .collect())
    }
}

pub struct XBooksTagsRepository {
    pool: SqlitePool,
}

impl XBooksTagsRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, book_id: i64, tag_id: i64) -> RitmoResult<()> {
        sqlx::query("INSERT INTO x_books_tags(book_id, tag_id) VALUES (?, ?)")
            .bind(book_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, book_id: i64, tag_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_books_tags WHERE book_id = ? AND tag_id = ?")
            .bind(book_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_book(&self, book_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT tag_id FROM x_books_tags WHERE book_id = ?")
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("tag_id")).collect())
    }

    pub async fn list_by_tag(&self, tag_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT book_id FROM x_books_tags WHERE tag_id = ?")
            .bind(tag_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("book_id")).collect())
    }
}

pub struct XBookLanguagesRepository {
    pool: SqlitePool,
}

impl XBookLanguagesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, book_id: i64, language_id: i64, language_role_id: i64) -> RitmoResult<()> {
        sqlx::query("INSERT INTO book_languages(book_id, language_id, role_id) VALUES (?, ?, ?)")
            .bind(book_id)
            .bind(language_id)
            .bind(language_role_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, book_id: i64, language_id: i64, language_role_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM book_languages WHERE book_id = ? AND language_id = ? AND role_id = ?")
            .bind(book_id)
            .bind(language_id)
            .bind(language_role_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_book(&self, book_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = sqlx::query("SELECT language_id, role_id FROM book_languages WHERE book_id = ?")
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("language_id"), row.get("role_id")))
            .collect())
    }

    pub async fn list_by_language(
        &self,
        language_id: i64,
        language_role_id: Option<i64>,
    ) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = match language_role_id {
            Some(role_id) => {
                sqlx::query(
                    "SELECT book_id, role_id FROM book_languages WHERE language_id = ? AND role_id = ?",
                )
                .bind(language_id)
                .bind(role_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?
            }
            None => {
                sqlx::query("SELECT book_id, role_id FROM book_languages WHERE language_id = ?")
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

pub struct XContentsPeopleRolesRepository {
    pool: SqlitePool,
}

impl XContentsPeopleRolesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, content_id: i64, person_id: i64, role_id: i64) -> RitmoResult<()> {
        sqlx::query(
            "INSERT INTO x_contents_people_roles(content_id, person_id, role_id) VALUES (?, ?, ?)",
        )
        .bind(content_id)
        .bind(person_id)
        .bind(role_id)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, content_id: i64, person_id: i64, role_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_contents_people_roles WHERE content_id = ? AND person_id = ? AND role_id = ?")
            .bind(content_id)
            .bind(person_id)
            .bind(role_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_content(&self, content_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows =
            sqlx::query("SELECT person_id, role_id FROM x_contents_people_roles WHERE content_id = ?")
                .bind(content_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("person_id"), row.get("role_id")))
            .collect())
    }

    pub async fn list_by_person(&self, person_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows =
            sqlx::query("SELECT content_id, role_id FROM x_contents_people_roles WHERE person_id = ?")
                .bind(person_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("content_id"), row.get("role_id")))
            .collect())
    }

    pub async fn list_by_role(&self, role_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows =
            sqlx::query("SELECT content_id, person_id FROM x_contents_people_roles WHERE role_id = ?")
                .bind(role_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("content_id"), row.get("person_id")))
            .collect())
    }
}

pub struct XContentsTagsRepository {
    pool: SqlitePool,
}

impl XContentsTagsRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, content_id: i64, tag_id: i64) -> RitmoResult<()> {
        sqlx::query("INSERT INTO x_contents_tags(content_id, tag_id) VALUES (?, ?)")
            .bind(content_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, content_id: i64, tag_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_contents_tags WHERE content_id = ? AND tag_id = ?")
            .bind(content_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_content(&self, content_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT tag_id FROM x_contents_tags WHERE content_id = ?")
            .bind(content_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("tag_id")).collect())
    }

    pub async fn list_by_tag(&self, tag_id: i64) -> RitmoResult<Vec<i64>> {
        let rows = sqlx::query("SELECT content_id FROM x_contents_tags WHERE tag_id = ?")
            .bind(tag_id)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows.into_iter().map(|row| row.get("content_id")).collect())
    }
}

pub struct XContentLanguagesRepository {
    pool: SqlitePool,
}

impl XContentLanguagesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, content_id: i64, language_id: i64, language_role_id: i64) -> RitmoResult<()> {
        sqlx::query("INSERT INTO content_languages(content_id, language_id, role_id) VALUES (?, ?, ?)")
            .bind(content_id)
            .bind(language_id)
            .bind(language_role_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, content_id: i64, language_id: i64, language_role_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM content_languages WHERE content_id = ? AND language_id = ? AND role_id = ?")
            .bind(content_id)
            .bind(language_id)
            .bind(language_role_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_content(&self, content_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows =
            sqlx::query("SELECT language_id, role_id FROM content_languages WHERE content_id = ?")
                .bind(content_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("language_id"), row.get("role_id")))
            .collect())
    }

    pub async fn list_by_language(
        &self,
        language_id: i64,
        language_role_id: Option<i64>,
    ) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = match language_role_id {
            Some(role_id) => {
                sqlx::query(
                    "SELECT content_id, role_id FROM content_languages WHERE language_id = ? AND role_id = ?",
                )
                .bind(language_id)
                .bind(role_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?
            }
            None => {
                sqlx::query(
                    "SELECT content_id, role_id FROM content_languages WHERE language_id = ?",
                )
                .bind(language_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?
            }
        };
        Ok(rows
            .into_iter()
            .map(|row| (row.get("content_id"), row.get("role_id")))
            .collect())
    }
}

pub struct XPersonLanguagesRepository {
    pool: SqlitePool,
}

impl XPersonLanguagesRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn create(&self, person_id: i64, language_id: i64, language_role_id: i64) -> RitmoResult<()> {
        sqlx::query("INSERT INTO person_languages(person_id, language_id, role_id) VALUES (?, ?, ?)")
            .bind(person_id)
            .bind(language_id)
            .bind(language_role_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(())
    }

    pub async fn delete(&self, person_id: i64, language_id: i64, language_role_id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM person_languages WHERE person_id = ? AND language_id = ? AND role_id = ?")
            .bind(person_id)
            .bind(language_id)
            .bind(language_role_id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_person(&self, person_id: i64) -> RitmoResult<Vec<(i64, i64)>> {
        let rows =
            sqlx::query("SELECT language_id, role_id FROM person_languages WHERE person_id = ?")
                .bind(person_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| (row.get("language_id"), row.get("role_id")))
            .collect())
    }

    pub async fn list_by_language(
        &self,
        language_id: i64,
        language_role_id: Option<i64>,
    ) -> RitmoResult<Vec<(i64, i64)>> {
        let rows = match language_role_id {
            Some(role_id) => {
                sqlx::query(
                    "SELECT person_id, role_id FROM person_languages WHERE language_id = ? AND role_id = ?",
                )
                .bind(language_id)
                .bind(role_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?
            }
            None => {
                sqlx::query(
                    "SELECT person_id, role_id FROM person_languages WHERE language_id = ?",
                )
                .bind(language_id)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?
            }
        };
        Ok(rows
            .into_iter()
            .map(|row| (row.get("person_id"), row.get("role_id")))
            .collect())
    }
}
