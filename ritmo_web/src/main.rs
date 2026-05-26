mod error;
mod handlers;
mod router;
mod state;

use std::env;
use std::net::SocketAddr;

use dotenv::dotenv;
use ritmo_errors::{RitmoErr, RitmoResult};
use tokio::net::TcpListener;

use crate::router::create_router;
use crate::state::{AppConfig, AppState};

#[tokio::main]
async fn main() -> RitmoResult<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(|_| RitmoErr::ConfigNotFound)?;
    let bind_addr: SocketAddr = env::var("WEB_BIND")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
        .parse()
        .map_err(|e| RitmoErr::ConfigParseError(format!("Invalid WEB_BIND: {e}")))?;

    let pool = ritmo_db::create_sqlite_pool(&database_url).await?;
    let state = AppState::new(
        pool,
        AppConfig {
            bind_addr,
            database_url,
        },
    );

    let listener = TcpListener::bind(state.config.bind_addr)
        .await
        .map_err(|e| RitmoErr::DatabaseConnection(format!("Bind error: {e}")))?;

    axum::serve(listener, create_router(state))
        .await
        .map_err(|e| RitmoErr::UnknownError(format!("Server error: {e}")))?;

    Ok(())
}
