use crate::support::{
    map_delete, map_insert, map_query, not_found, partial_date_from_parts, partial_date_to_parts,
    RepositoryContext,
};
use ritmo_domain::{PartialDate, Person};
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

pub struct PersonRepository {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct PersonDetailData {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names: Option<String>,
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub birth_date: Option<PartialDate>,
    pub death_date: Option<PartialDate>,
    pub biography: Option<String>,
    pub verified: bool,
    pub aliases: Vec<String>,
    pub places: Vec<(String, Option<String>, Option<String>, Option<String>)>,
    pub languages: Vec<(String, Option<String>, String)>,
    pub books: Vec<(i64, String, String)>,
    pub contents: Vec<(i64, String, String)>,
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
            "INSERT OR IGNORE INTO d_people(name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography, verified) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        .bind(i64::from(person.verified))
        .execute(&self.pool)
        .await
        .map_err(map_insert)?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get(&self, id: i64) -> RitmoResult<Person> {
        let row = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography, verified FROM d_people WHERE id = ?")
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
            verified: row.get::<i64, _>("verified") != 0,
        })
    }

    pub async fn get_detail(&self, id: i64) -> RitmoResult<PersonDetailData> {
        let row = sqlx::query(
            "SELECT
                id,
                name,
                display_name,
                given_name,
                surname,
                middle_names,
                title,
                suffix,
                birth_date_year,
                birth_date_month,
                birth_date_day,
                birth_date_circa,
                death_date_year,
                death_date_month,
                death_date_day,
                death_date_circa,
                biography,
                verified
             FROM d_people
             WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_query)?
        .ok_or_else(not_found)?;

        let aliases = sqlx::query(
            "SELECT name
             FROM d_aliases
             WHERE person_id = ?
             ORDER BY name COLLATE NOCASE",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?
        .into_iter()
        .map(|row| row.get::<String, _>("name"))
        .collect();

        let places = sqlx::query(
            "SELECT pt.key AS place_type_key, p.continent, p.country, p.city
             FROM x_person_places xpp
             INNER JOIN d_places p ON p.id = xpp.place_id
             INNER JOIN s_place_types pt ON pt.id = xpp.place_type_id
             WHERE xpp.person_id = ?
             ORDER BY pt.key COLLATE NOCASE, p.city COLLATE NOCASE, p.country COLLATE NOCASE, p.continent COLLATE NOCASE",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(map_query)?
        .into_iter()
        .map(|row| {
            (
                row.get::<String, _>("place_type_key"),
                row.get::<Option<String>, _>("continent"),
                row.get::<Option<String>, _>("country"),
                row.get::<Option<String>, _>("city"),
            )
        })
        .collect();

        let languages = sqlx::query(
            "SELECT l.official_name, l.iso_code_2char, lr.code AS role_code
             FROM x_person_languages xpl
             INNER JOIN d_languages l ON l.id = xpl.language_id
             INNER JOIN s_person_language_roles lr ON lr.id = xpl.role_id
             WHERE xpl.person_id = ?
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

        let books = sqlx::query(
            "SELECT b.id, b.name, r.key AS role_key
             FROM x_books_people_roles xbpr
             INNER JOIN d_books b ON b.id = xbpr.book_id
             INNER JOIN d_roles r ON r.id = xbpr.role_id
             WHERE xbpr.person_id = ?
             ORDER BY r.key COLLATE NOCASE, b.name COLLATE NOCASE",
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

        let contents = sqlx::query(
            "SELECT c.id, c.name, r.key AS role_key
             FROM x_contents_people_roles xcpr
             INNER JOIN d_contents c ON c.id = xcpr.content_id
             INNER JOIN d_roles r ON r.id = xcpr.role_id
             WHERE xcpr.person_id = ?
             ORDER BY r.key COLLATE NOCASE, c.name COLLATE NOCASE",
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

        Ok(PersonDetailData {
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
            verified: row.get::<i64, _>("verified") != 0,
            aliases,
            places,
            languages,
            books,
            contents,
        })
    }

    pub async fn get_by_name(&self, name: &str) -> RitmoResult<Option<Person>> {
        let row = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography, verified FROM d_people WHERE name = ?")
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
            verified: value.get::<i64, _>("verified") != 0,
        }))
    }

    pub async fn update(&self, person: &Person) -> RitmoResult<()> {
        let (birth_year, birth_month, birth_day, birth_circa) =
            partial_date_to_parts(&person.birth_date);
        let (death_year, death_month, death_day, death_circa) =
            partial_date_to_parts(&person.death_date);

        sqlx::query("UPDATE d_people SET name = ?, display_name = ?, given_name = ?, surname = ?, middle_names = ?, title = ?, suffix = ?, birth_date_year = ?, birth_date_month = ?, birth_date_day = ?, birth_date_circa = ?, death_date_year = ?, death_date_month = ?, death_date_day = ?, death_date_circa = ?, biography = ?, verified = ? WHERE id = ?")
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
            .bind(i64::from(person.verified))
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
        let rows = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography, verified FROM d_people ORDER BY name")
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
                verified: row.get::<i64, _>("verified") != 0,
            })
            .collect())
    }

    pub async fn search(&self, query: &str) -> RitmoResult<Vec<Person>> {
        let pattern = format!("%{query}%");
        let rows = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography, verified FROM d_people WHERE name LIKE ? COLLATE NOCASE ORDER BY name")
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
                verified: row.get::<i64, _>("verified") != 0,
            })
            .collect())
    }

    pub async fn list_all_for_display(
        pool: &SqlitePool,
    ) -> RitmoResult<Vec<(i64, String, Option<String>, Option<i64>, Option<i64>)>> {
        let rows = sqlx::query(
            "SELECT
                id,
                name,
                display_name,
                birth_date_year,
                death_date_year
            FROM d_people
            ORDER BY name COLLATE NOCASE",
        )
        .fetch_all(pool)
        .await
        .map_err(map_query)?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.get::<i64, _>("id"),
                    row.get::<String, _>("name"),
                    row.get::<Option<String>, _>("display_name"),
                    row.get::<Option<i64>, _>("birth_date_year"),
                    row.get::<Option<i64>, _>("death_date_year"),
                )
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
            verified: false,
        };
        let id = self.save(&created).await?;
        self.get(id).await
    }
}
