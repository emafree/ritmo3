use ritmo_core::CoreContext;
use ritmo_errors::RitmoResult;
use tera::Tera;

#[derive(Clone)]
pub struct AppConfig {
    pub bind_addr: std::net::SocketAddr,
    #[allow(dead_code)]
    pub database_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub core: CoreContext,
    pub config: AppConfig,
    pub tera: Tera,
}

impl AppState {
    pub fn new(core: CoreContext, config: AppConfig, tera: Tera) -> Self {
        Self { core, config, tera }
    }
}

pub fn load_tera() -> RitmoResult<Tera> {
    Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html"))
        .map_err(|e| ritmo_errors::RitmoErr::UnknownError(format!("Template error: {e}")))
}
