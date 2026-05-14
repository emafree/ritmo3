use std::env;

use dotenv::dotenv;
use ritmo_errors::{RitmoErr, RitmoResult};

#[tokio::main]
async fn main() -> RitmoResult<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(|_| RitmoErr::ConfigNotFound)?;

    let pool = ritmo_db::create_sqlite_pool(&database_url).await?;
    ritmo_tui::run()?;
    drop(pool);

    Ok(())
}
