use crate::support::{
    map_delete, map_insert, map_query, not_found, partial_date_from_parts, partial_date_to_parts,
    RepositoryContext,
};
use ritmo_domain::{
    Alias, Book, Content, Format, Genre, Language, Person, Place, PlaceType, Publisher, Role,
    Series, Tag,
};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct AliasRepository {
    pool: SqlitePool,
}

impl AliasRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, alias: &Alias) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT OR IGNORE INTO d_aliases(name, person_id) VALUES (?, ?)")
            .bind(&alias.alternative_name)
            .bind(alias.person_id)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Alias> {
        let row = sqlx::query("SELECT id, name, person_id FROM d_aliases WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(Alias {
            id: row.get("id"),
            alternative_name: row.get("name"),
            person_id: row.get("person_id"),
        })
    }

    pub async fn update(&self, alias: &Alias) -> RitmoResult<()> {
        sqlx::query("UPDATE d_aliases SET name = ?, person_id = ? WHERE id = ?")
            .bind(&alias.alternative_name)
            .bind(alias.person_id)
            .bind(alias.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM d_aliases WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Alias>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query(
            "SELECT id, name, person_id FROM d_aliases WHERE name LIKE ? COLLATE NOCASE ORDER BY name",
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Alias {
                id: row.get("id"),
                alternative_name: row.get("name"),
                person_id: row.get("person_id"),
            })
            .collect())
    }

    pub async fn list_by_person(&self, person_id: i64) -> RitmoResult<Vec<Alias>> {
        let rows = sqlx::query(
            "SELECT id, name, person_id FROM d_aliases WHERE person_id = ? ORDER BY name",
        )
        .bind(person_id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Alias {
                id: row.get("id"),
                alternative_name: row.get("name"),
                person_id: row.get("person_id"),
            })
            .collect())
    }

    pub async fn get_by_person_and_name(
        &self,
        person_id: i64,
        name: &str,
    ) -> RitmoResult<Option<Alias>> {
        let row =
            sqlx::query("SELECT id, name, person_id FROM d_aliases WHERE person_id = ? AND name = ?")
                .bind(person_id)
                .bind(name)
                .fetch_optional(&self.pool)
                .await
                .map_err(map_query)?;

        Ok(row.map(|value| Alias {
            id: value.get("id"),
            alternative_name: value.get("name"),
            person_id: value.get("person_id"),
        }))
    }
}

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
            "INSERT OR IGNORE INTO d_books(name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes) VALUES (?, ?, ?, ?, ?, ?, ?)",
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
        let row = sqlx::query("SELECT id, name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_books WHERE id = ?")
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
            "UPDATE d_books SET name = ?, isbn = ?, publication_date_year = ?, publication_date_month = ?, publication_date_day = ?, publication_date_circa = ?, notes = ? WHERE id = ?",
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
        sqlx::query("DELETE FROM d_books WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Book>> {
        let rows = sqlx::query("SELECT id, name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_books ORDER BY name")
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
        let rows = sqlx::query("SELECT id, name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_books WHERE name LIKE ? COLLATE NOCASE ORDER BY name")
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

    pub async fn get_or_create(&self, title: &str) -> RitmoResult<Book> {
        if let Some(row) = sqlx::query("SELECT id, name, isbn, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_books WHERE name = ?")
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
            "INSERT OR IGNORE INTO d_contents(name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes) VALUES (?, ?, ?, ?, ?, ?)",
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
        let row = sqlx::query("SELECT id, name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_contents WHERE id = ?")
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
            "UPDATE d_contents SET name = ?, publication_date_year = ?, publication_date_month = ?, publication_date_day = ?, publication_date_circa = ?, notes = ? WHERE id = ?",
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
        sqlx::query("DELETE FROM d_contents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Content>> {
        let rows = sqlx::query("SELECT id, name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_contents ORDER BY name")
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
        let rows = sqlx::query("SELECT id, name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_contents WHERE name LIKE ? COLLATE NOCASE ORDER BY name")
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
        if let Some(row) = sqlx::query("SELECT id, name, publication_date_year, publication_date_month, publication_date_day, publication_date_circa, notes FROM d_contents WHERE name = ?")
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

macro_rules! define_key_repo {
    ($repo:ident, $entity:ident, $table:literal, $field:ident, $column:literal) => {
        pub struct $repo {
            pool: SqlitePool,
        }

        impl $repo {
            pub fn new(ctx: &RepositoryContext) -> Self {
                Self {
                    pool: ctx.pool().clone(),
                }
            }

            pub async fn save(&self, item: &$entity) -> RitmoResult<i64> {
                let result = sqlx::query(concat!(
                    "INSERT OR IGNORE INTO ",
                    $table,
                    "(",
                    $column,
                    ") VALUES (?)"
                ))
                .bind(&item.$field)
                .execute(&self.pool)
                .await
                .map_err(map_insert)?;
                Ok(result.last_insert_rowid())
            }

            pub async fn get(&self, id: i64) -> RitmoResult<$entity> {
                let row = sqlx::query(concat!(
                    "SELECT id, ",
                    $column,
                    " FROM ",
                    $table,
                    " WHERE id = ?"
                ))
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(map_query)?
                .ok_or_else(not_found)?;
                Ok($entity {
                    id: row.get("id"),
                    $field: row.get($column),
                })
            }

            pub async fn update(&self, item: &$entity) -> RitmoResult<()> {
                sqlx::query(concat!(
                    "UPDATE ",
                    $table,
                    " SET ",
                    $column,
                    " = ? WHERE id = ?"
                ))
                .bind(&item.$field)
                .bind(item.id)
                .execute(&self.pool)
                .await
                .map_err(map_query)?;
                Ok(())
            }

            pub async fn delete(&self, id: i64) -> RitmoResult<()> {
                sqlx::query(concat!("DELETE FROM ", $table, " WHERE id = ?"))
                    .bind(id)
                    .execute(&self.pool)
                    .await
                    .map_err(map_delete)?;
                Ok(())
            }

            pub async fn list_all(&self) -> RitmoResult<Vec<$entity>> {
                let rows = sqlx::query(concat!(
                    "SELECT id, ",
                    $column,
                    " FROM ",
                    $table,
                    " ORDER BY ",
                    $column
                ))
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
                Ok(rows
                    .into_iter()
                    .map(|row| $entity {
                        id: row.get("id"),
                        $field: row.get($column),
                    })
                    .collect())
            }

            pub async fn search(&self, query: &str) -> RitmoResult<Vec<$entity>> {
                let pattern = format!("%{query}%");
                let rows = sqlx::query(concat!(
                    "SELECT id, ",
                    $column,
                    " FROM ",
                    $table,
                    " WHERE ",
                    $column,
                    " LIKE ? COLLATE NOCASE ORDER BY ",
                    $column
                ))
                .bind(pattern)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
                Ok(rows
                    .into_iter()
                    .map(|row| $entity {
                        id: row.get("id"),
                        $field: row.get($column),
                    })
                    .collect())
            }

            pub async fn get_or_create(&self, value: &str) -> RitmoResult<$entity> {
                if let Some(row) = sqlx::query(concat!(
                    "SELECT id, ",
                    $column,
                    " FROM ",
                    $table,
                    " WHERE ",
                    $column,
                    " = ?"
                ))
                .bind(value)
                .fetch_optional(&self.pool)
                .await
                .map_err(map_query)?
                {
                    return Ok($entity {
                        id: row.get("id"),
                        $field: row.get($column),
                    });
                }

                let created = $entity {
                    id: 0,
                    $field: value.to_string(),
                };
                let id = self.save(&created).await?;
                self.get(id).await
            }
        }
    };
}

define_key_repo!(FormatRepository, Format, "formats", i18n_key, "key");
define_key_repo!(GenreRepository, Genre, "genres", i18n_key, "key");
define_key_repo!(RoleRepository, Role, "roles", i18n_key, "key");

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
            "INSERT OR IGNORE INTO d_languages(iso_code_2char, iso_code_3char, official_name) VALUES (?, ?, ?)",
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
            "SELECT id, iso_code_2char, iso_code_3char, official_name FROM d_languages WHERE id = ?",
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
            "UPDATE d_languages SET iso_code_2char = ?, iso_code_3char = ?, official_name = ? WHERE id = ?",
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
        sqlx::query("DELETE FROM d_languages WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Language>> {
        let rows = sqlx::query(
            "SELECT id, iso_code_2char, iso_code_3char, official_name FROM d_languages ORDER BY official_name",
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
            "SELECT id, iso_code_2char, iso_code_3char, official_name FROM d_languages WHERE official_name LIKE ? COLLATE NOCASE ORDER BY official_name",
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
            "SELECT id, iso_code_2char, iso_code_3char, official_name FROM d_languages WHERE official_name = ?",
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

pub struct PersonRepository {
    pool: SqlitePool,
}

impl PersonRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, person: &Person) -> RitmoResult<i64> {
        let (birth_year, birth_month, birth_day, birth_circa) =
            partial_date_to_parts(&person.birth_date);
        let (death_year, death_month, death_day, death_circa) =
            partial_date_to_parts(&person.death_date);

        let result = sqlx::query(
            "INSERT OR IGNORE INTO d_people(name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&person.name)
        .bind(&person.display_name)
        .bind(&person.given_name)
        .bind(&person.surname)
        .bind(&person.middle_names)
        .bind(&person.title)
        .bind(&person.suffix)
        .bind(birth_year)
        .bind(birth_month)
        .bind(birth_day)
        .bind(birth_circa)
        .bind(death_year)
        .bind(death_month)
        .bind(death_day)
        .bind(death_circa)
        .bind(&person.biography)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Person> {
        let row = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography FROM d_people WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(Person {
            id: row.get("id"),
            name: row.get("name"),
            display_name: row.get("display_name"),
            given_name: row.get("given_name"),
            surname: row.get("surname"),
            middle_names: row.get("middle_names"),
            title: row.get("title"),
            suffix: row.get("suffix"),
            birth_date: partial_date_from_parts(
                row.get("birth_date_year"),
                row.get("birth_date_month"),
                row.get("birth_date_day"),
                row.get::<i64, _>("birth_date_circa"),
            ),
            death_date: partial_date_from_parts(
                row.get("death_date_year"),
                row.get("death_date_month"),
                row.get("death_date_day"),
                row.get::<i64, _>("death_date_circa"),
            ),
            biography: row.get("biography"),
        })
    }

    pub async fn get_by_name(&self, name: &str) -> RitmoResult<Option<Person>> {
        let row = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography FROM d_people WHERE name = ?")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?;

        Ok(row.map(|value| Person {
            id: value.get("id"),
            name: value.get("name"),
            display_name: value.get("display_name"),
            given_name: value.get("given_name"),
            surname: value.get("surname"),
            middle_names: value.get("middle_names"),
            title: value.get("title"),
            suffix: value.get("suffix"),
            birth_date: partial_date_from_parts(
                value.get("birth_date_year"),
                value.get("birth_date_month"),
                value.get("birth_date_day"),
                value.get::<i64, _>("birth_date_circa"),
            ),
            death_date: partial_date_from_parts(
                value.get("death_date_year"),
                value.get("death_date_month"),
                value.get("death_date_day"),
                value.get::<i64, _>("death_date_circa"),
            ),
            biography: value.get("biography"),
        }))
    }

    pub async fn update(&self, person: &Person) -> RitmoResult<()> {
        let (birth_year, birth_month, birth_day, birth_circa) =
            partial_date_to_parts(&person.birth_date);
        let (death_year, death_month, death_day, death_circa) =
            partial_date_to_parts(&person.death_date);

        sqlx::query("UPDATE d_people SET name = ?, display_name = ?, given_name = ?, surname = ?, middle_names = ?, title = ?, suffix = ?, birth_date_year = ?, birth_date_month = ?, birth_date_day = ?, birth_date_circa = ?, death_date_year = ?, death_date_month = ?, death_date_day = ?, death_date_circa = ?, biography = ? WHERE id = ?")
            .bind(&person.name)
            .bind(&person.display_name)
            .bind(&person.given_name)
            .bind(&person.surname)
            .bind(&person.middle_names)
            .bind(&person.title)
            .bind(&person.suffix)
            .bind(birth_year)
            .bind(birth_month)
            .bind(birth_day)
            .bind(birth_circa)
            .bind(death_year)
            .bind(death_month)
            .bind(death_day)
            .bind(death_circa)
            .bind(&person.biography)
            .bind(person.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM d_people WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Person>> {
        let rows = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography FROM d_people ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Person {
                id: row.get("id"),
                name: row.get("name"),
                display_name: row.get("display_name"),
                given_name: row.get("given_name"),
                surname: row.get("surname"),
                middle_names: row.get("middle_names"),
                title: row.get("title"),
                suffix: row.get("suffix"),
                birth_date: partial_date_from_parts(
                    row.get("birth_date_year"),
                    row.get("birth_date_month"),
                    row.get("birth_date_day"),
                    row.get::<i64, _>("birth_date_circa"),
                ),
                death_date: partial_date_from_parts(
                    row.get("death_date_year"),
                    row.get("death_date_month"),
                    row.get("death_date_day"),
                    row.get::<i64, _>("death_date_circa"),
                ),
                biography: row.get("biography"),
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Person>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography FROM d_people WHERE name LIKE ? COLLATE NOCASE ORDER BY name")
            .bind(pattern)
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| Person {
                id: row.get("id"),
                name: row.get("name"),
                display_name: row.get("display_name"),
                given_name: row.get("given_name"),
                surname: row.get("surname"),
                middle_names: row.get("middle_names"),
                title: row.get("title"),
                suffix: row.get("suffix"),
                birth_date: partial_date_from_parts(
                    row.get("birth_date_year"),
                    row.get("birth_date_month"),
                    row.get("birth_date_day"),
                    row.get::<i64, _>("birth_date_circa"),
                ),
                death_date: partial_date_from_parts(
                    row.get("death_date_year"),
                    row.get("death_date_month"),
                    row.get("death_date_day"),
                    row.get::<i64, _>("death_date_circa"),
                ),
                biography: row.get("biography"),
            })
            .collect())
    }

    pub async fn get_or_create(&self, name: &str) -> RitmoResult<Person> {
        if let Some(person) = self.get_by_name(name).await? {
            return Ok(person);
        }

        let created = Person {
            id: 0,
            name: name.to_string(),
            display_name: None,
            given_name: None,
            surname: None,
            middle_names: None,
            title: None,
            suffix: None,
            birth_date: None,
            death_date: None,
            biography: None,
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}

pub struct PlaceTypeRepository {
    pool: SqlitePool,
}

impl PlaceTypeRepository {
    pub fn new(ctx: &RepositoryContext) -> Self {
        Self {
            pool: ctx.pool().clone(),
        }
    }

    pub async fn save(&self, place_type: &PlaceType) -> RitmoResult<i64> {
        let result = sqlx::query("INSERT OR IGNORE INTO s_place_types(key) VALUES (?)")
            .bind(&place_type.name)
            .execute(&self.pool)
            .await
            .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<PlaceType> {
        let row = sqlx::query("SELECT id, key FROM s_place_types WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_query)?
            .ok_or_else(not_found)?;
        Ok(PlaceType {
            id: row.get("id"),
            name: row.get("key"),
        })
    }

    pub async fn update(&self, place_type: &PlaceType) -> RitmoResult<()> {
        sqlx::query("UPDATE s_place_types SET key = ? WHERE id = ?")
            .bind(&place_type.name)
            .bind(place_type.id)
            .execute(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM s_place_types WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<PlaceType>> {
        let rows = sqlx::query("SELECT id, key FROM s_place_types ORDER BY key")
            .fetch_all(&self.pool)
            .await
            .map_err(map_query)?;
        Ok(rows
            .into_iter()
            .map(|row| PlaceType {
                id: row.get("id"),
                name: row.get("key"),
            })
            .collect())
    }
}

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
            "INSERT OR IGNORE INTO x_person_places(person_id, place_type_id, place_name) VALUES (?, ?, ?)",
        )
        .bind(place.person_id)
        .bind(place.place_type_id)
        .bind(&place.name)
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Place> {
        let row = sqlx::query(
            "SELECT id, place_name, place_type_id, person_id FROM x_person_places WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?
        .ok_or_else(not_found)?;

        Ok(Place {
            id: row.get("id"),
            name: row.get("place_name"),
            place_type_id: row.get("place_type_id"),
            person_id: row.get("person_id"),
        })
    }

    pub async fn update(&self, place: &Place) -> RitmoResult<()> {
        sqlx::query(
            "UPDATE x_person_places SET place_name = ?, place_type_id = ?, person_id = ? WHERE id = ?",
        )
        .bind(&place.name)
        .bind(place.place_type_id)
        .bind(place.person_id)
        .bind(place.id)
        .execute(&self.pool)
        .await
        .map_err(map_query)?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> RitmoResult<()> {
        sqlx::query("DELETE FROM x_person_places WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_by_person(&self, person_id: i64) -> RitmoResult<Vec<Place>> {
        let rows = sqlx::query(
            "SELECT id, place_name, place_type_id, person_id FROM x_person_places WHERE person_id = ? ORDER BY place_name",
        )
        .bind(person_id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| Place {
                id: row.get("id"),
                name: row.get("place_name"),
                place_type_id: row.get("place_type_id"),
                person_id: row.get("person_id"),
            })
            .collect())
    }
}

macro_rules! define_name_repo {
    ($repo:ident, $entity:ident, $table:literal) => {
        pub struct $repo {
            pool: SqlitePool,
        }

        impl $repo {
            pub fn new(ctx: &RepositoryContext) -> Self {
                Self {
                    pool: ctx.pool().clone(),
                }
            }

            pub async fn save(&self, item: &$entity) -> RitmoResult<i64> {
                let result = sqlx::query(concat!(
                    "INSERT OR IGNORE INTO ",
                    $table,
                    "(name) VALUES (?)"
                ))
                .bind(&item.name)
                .execute(&self.pool)
                .await
                .map_err(map_insert)?;
                Ok(result.last_insert_rowid())
            }

            pub async fn get(&self, id: i64) -> RitmoResult<$entity> {
                let row = sqlx::query(concat!("SELECT id, name FROM ", $table, " WHERE id = ?"))
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(map_query)?
                    .ok_or_else(not_found)?;
                Ok($entity {
                    id: row.get("id"),
                    name: row.get("name"),
                })
            }

            pub async fn update(&self, item: &$entity) -> RitmoResult<()> {
                sqlx::query(concat!("UPDATE ", $table, " SET name = ? WHERE id = ?"))
                    .bind(&item.name)
                    .bind(item.id)
                    .execute(&self.pool)
                    .await
                    .map_err(map_query)?;
                Ok(())
            }

            pub async fn delete(&self, id: i64) -> RitmoResult<()> {
                sqlx::query(concat!("DELETE FROM ", $table, " WHERE id = ?"))
                    .bind(id)
                    .execute(&self.pool)
                    .await
                    .map_err(map_delete)?;
                Ok(())
            }

            pub async fn list_all(&self) -> RitmoResult<Vec<$entity>> {
                let rows = sqlx::query(concat!("SELECT id, name FROM ", $table, " ORDER BY name"))
                    .fetch_all(&self.pool)
                    .await
                    .map_err(map_query)?;
                Ok(rows
                    .into_iter()
                    .map(|row| $entity {
                        id: row.get("id"),
                        name: row.get("name"),
                    })
                    .collect())
            }

            pub async fn search(&self, query: &str) -> RitmoResult<Vec<$entity>> {
                let pattern = format!("%{query}%");
                let rows = sqlx::query(concat!(
                    "SELECT id, name FROM ",
                    $table,
                    " WHERE name LIKE ? COLLATE NOCASE ORDER BY name"
                ))
                .bind(pattern)
                .fetch_all(&self.pool)
                .await
                .map_err(map_query)?;
                Ok(rows
                    .into_iter()
                    .map(|row| $entity {
                        id: row.get("id"),
                        name: row.get("name"),
                    })
                    .collect())
            }

            pub async fn get_or_create(&self, value: &str) -> RitmoResult<$entity> {
                if let Some(row) =
                    sqlx::query(concat!("SELECT id, name FROM ", $table, " WHERE name = ?"))
                        .bind(value)
                        .fetch_optional(&self.pool)
                        .await
                        .map_err(map_query)?
                {
                    return Ok($entity {
                        id: row.get("id"),
                        name: row.get("name"),
                    });
                }

                let created = $entity {
                    id: 0,
                    name: value.to_string(),
                };
                let id = self.save(&created).await?;
                self.get(id).await
            }
        }
    };
}

define_name_repo!(PublisherRepository, Publisher, "publishers");
define_name_repo!(SeriesRepository, Series, "series");
define_name_repo!(TagRepository, Tag, "tags");
