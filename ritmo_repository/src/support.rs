use ritmo_domain::PartialDate;
use ritmo_errors::{RitmoErr, RitmoResult};
use sqlx::SqlitePool;

pub async fn create_pool(database_url: &str) -> RitmoResult<SqlitePool> {
    ritmo_db::create_sqlite_pool(database_url).await
}

#[derive(Clone)]
pub struct RepositoryContext {
    pool: SqlitePool,
}

impl RepositoryContext {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

pub(crate) fn partial_date_from_parts(
    year: Option<i32>,
    month: Option<i64>,
    day: Option<i64>,
    circa: i64,
) -> Option<PartialDate> {
    if year.is_none() && month.is_none() && day.is_none() && circa == 0 {
        return None;
    }

    Some(PartialDate {
        year,
        month: month.and_then(|value| u8::try_from(value).ok()),
        day: day.and_then(|value| u8::try_from(value).ok()),
        circa: circa != 0,
    })
}

pub(crate) fn partial_date_to_parts(
    date: &Option<PartialDate>,
) -> (Option<i32>, Option<i64>, Option<i64>, i64) {
    match date {
        Some(value) => (
            value.year,
            value.month.map(i64::from),
            value.day.map(i64::from),
            i64::from(value.circa),
        ),
        None => (None, None, None, 0),
    }
}

pub(crate) fn not_found() -> RitmoErr {
    RitmoErr::RecordNotFound
}

pub(crate) fn map_query(err: sqlx::Error) -> RitmoErr {
    RitmoErr::DatabaseQuery(err.to_string())
}

pub(crate) fn map_insert(err: sqlx::Error) -> RitmoErr {
    RitmoErr::DatabaseInsert(err.to_string())
}

pub(crate) fn map_delete(err: sqlx::Error) -> RitmoErr {
    RitmoErr::DatabaseDelete(err.to_string())
}
