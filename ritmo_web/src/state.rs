use ritmo_repository::RepositoryContext;
use tera::Tera;

#[derive(Clone)]
pub struct AppConfig {
    pub bind_addr: std::net::SocketAddr,
    #[allow(dead_code)]
    pub database_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub repo: RepositoryContext,
    pub config: AppConfig,
    pub tera: Tera,
}

impl AppState {
    pub fn new(repo: RepositoryContext, config: AppConfig, tera: Tera) -> Self {
        Self { repo, config, tera }
    }
}
