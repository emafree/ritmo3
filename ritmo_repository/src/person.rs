use crate::support::{
    map_delete, map_insert, map_query, not_found, partial_date_from_parts, partial_date_to_parts,
    RepositoryContext,
};
use ritmo_domain::Person;
use ritmo_errors::RitmoResult;
use sqlx::{Row, SqlitePool};

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
            "INSERT INTO people(name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        let row = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography FROM people WHERE id = ?")
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
        let row = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography FROM people WHERE name = ?")
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

        sqlx::query("UPDATE people SET name = ?, display_name = ?, given_name = ?, surname = ?, middle_names = ?, title = ?, suffix = ?, birth_date_year = ?, birth_date_month = ?, birth_date_day = ?, birth_date_circa = ?, death_date_year = ?, death_date_month = ?, death_date_day = ?, death_date_circa = ?, biography = ? WHERE id = ?")
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
        sqlx::query("DELETE FROM people WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(map_delete)?;
        Ok(())
    }

    pub async fn list_all(&self) -> RitmoResult<Vec<Person>> {
        let rows = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography FROM people ORDER BY name")
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
        let rows = sqlx::query("SELECT id, name, display_name, given_name, surname, middle_names, title, suffix, birth_date_year, birth_date_month, birth_date_day, birth_date_circa, death_date_year, death_date_month, death_date_day, death_date_circa, biography FROM people WHERE name LIKE ? COLLATE NOCASE ORDER BY name")
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
