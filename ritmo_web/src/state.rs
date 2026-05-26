use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppConfig {
    pub bind_addr: std::net::SocketAddr,
    pub database_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(pool: SqlitePool, config: AppConfig) -> Self {
        Self { pool, config }
    }
}
