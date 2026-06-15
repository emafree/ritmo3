pub mod book;
pub mod content;
pub mod content_type;
pub mod format;
pub mod language;
pub mod person;
pub mod place;
pub mod publisher;
pub mod rel_book_content;
pub mod rel_book_language;
pub mod rel_book_person;
pub mod rel_book_tag;
pub mod rel_content_language;
pub mod rel_content_person;
pub mod rel_content_tag;
pub mod rel_person_language;
pub mod role;
pub mod series;
pub mod tag;

use ritmo_repository::RepositoryContext;
use ritmo_errors::RitmoResult;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct CoreContext {
    pub(crate) ctx: RepositoryContext,
}

impl CoreContext {
    pub fn new(ctx: RepositoryContext) -> Self {
        Self { ctx }
    }

    pub fn from_pool(pool: SqlitePool) -> Self {
        Self {
            ctx: RepositoryContext::new(pool),
        }
    }

    pub async fn connect(database_url: &str) -> RitmoResult<Self> {
        let pool = ritmo_repository::create_pool(database_url).await?;
        Ok(Self::from_pool(pool))
    }
}
