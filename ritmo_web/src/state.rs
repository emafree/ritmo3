use ritmo_repository::RepositoryContext;

#[derive(Clone)]
pub struct AppConfig {
    pub bind_addr: std::net::SocketAddr,
    pub database_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub repo: RepositoryContext,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(repo: RepositoryContext, config: AppConfig) -> Self {
        Self { repo, config }
    }
}
