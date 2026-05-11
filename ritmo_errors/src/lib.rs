#![allow(unused)]
// ritmo_errors/src/lib.rs

pub mod reporter;

pub type RitmoResult<T> = Result<T, RitmoErr>;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RitmoErr {
    // Database
    #[error("Database connection error: {0}")]
    DatabaseConnection(String),
    #[error("Database query error: {0}")]
    DatabaseQuery(String),
    #[error("Database migration error: {0}")]
    DatabaseMigration(String),
    #[error("Database insert error: {0}")]
    DatabaseInsert(String),
    #[error("Database delete error: {0}")]
    DatabaseDelete(String),
    #[error("Database transaction error: {0}")]
    DatabaseTransaction(String),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Data integrity error: {0}")]
    DataIntegrity(String),

    // File system
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("File access error: {0}")]
    FileAccess(#[from] std::io::Error),
    #[error("Path error: {0}")]
    PathError(String),

    // Import/Export
    #[error("Import error: {0}")]
    ImportError(String),
    #[error("Export error: {0}")]
    ExportError(String),

    // Configuration
    #[error("Configuration not found")]
    ConfigNotFound,
    #[error("Configuration parse error: {0}")]
    ConfigParseError(String),

    // Domain
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Name parsing error: {0}")]
    NameParsingError(String),
    #[error("Merge error: {0}")]
    MergeError(String),

    // ML / Serialization
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("ML error: {0}")]
    MLError(String),

    // Generic
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl From<sqlx::Error> for RitmoErr {
    fn from(err: sqlx::Error) -> Self {
        RitmoErr::DatabaseConnection(format!("SQLx error: {}", err))
    }
}

impl From<serde_json::Error> for RitmoErr {
    fn from(err: serde_json::Error) -> Self {
        RitmoErr::SerializationError(format!("JSON error: {}", err))
    }
}

impl From<toml::de::Error> for RitmoErr {
    fn from(err: toml::de::Error) -> Self {
        RitmoErr::ConfigParseError(format!("TOML parse error: {}", err))
    }
}

impl From<toml::ser::Error> for RitmoErr {
    fn from(err: toml::ser::Error) -> Self {
        RitmoErr::ConfigParseError(format!("TOML serialization error: {}", err))
    }
}
