use ritmo_errors::{RitmoErr, RitmoResult};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

const SCHEMA_SQL: &str = include_str!("../schema/schema.sql");
const SEED_LOOKUPS_SQL: &str = include_str!("../schema/seed_lookups.sql");
const SEED_PAGE_FIELDS_SQL: &str = include_str!("../schema/seed_page_fields.sql");

pub async fn create_sqlite_pool(database_url: &str) -> RitmoResult<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .connect(database_url)
        .await
        .map_err(|err| RitmoErr::DatabaseConnection(format!("failed to connect to sqlite: {err}")))?;

    sqlx::raw_sql(SCHEMA_SQL)
        .execute(&pool)
        .await
        .map_err(|err| RitmoErr::DatabaseMigration(format!("failed to run schema.sql: {err}")))?;

    sqlx::raw_sql(SEED_LOOKUPS_SQL)
        .execute(&pool)
        .await
        .map_err(|err| RitmoErr::DatabaseMigration(format!("failed to run seed_lookups.sql: {err}")))?;

    sqlx::raw_sql(SEED_PAGE_FIELDS_SQL)
        .execute(&pool)
        .await
        .map_err(|err| RitmoErr::DatabaseMigration(format!("failed to run seed_page_fields.sql: {err}")))?;

    Ok(pool)
}
