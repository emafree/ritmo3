use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use ritmo_errors::RitmoErr;
use serde_json::json;

pub struct WebError(pub RitmoErr);

impl From<RitmoErr> for WebError {
    fn from(value: RitmoErr) -> Self {
        Self(value)
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            RitmoErr::RecordNotFound => StatusCode::NOT_FOUND,
            RitmoErr::InvalidInput(_) | RitmoErr::ConfigParseError(_) => StatusCode::BAD_REQUEST,
            RitmoErr::ConfigNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            RitmoErr::DatabaseConnection(_)
            | RitmoErr::DatabaseQuery(_)
            | RitmoErr::DatabaseMigration(_)
            | RitmoErr::DatabaseInsert(_)
            | RitmoErr::DatabaseDelete(_)
            | RitmoErr::DatabaseTransaction(_)
            | RitmoErr::DataIntegrity(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RitmoErr::FileNotFound(_) | RitmoErr::FileAccess(_) | RitmoErr::PathError(_) => {
                StatusCode::BAD_REQUEST
            }
            RitmoErr::ImportError(_)
            | RitmoErr::ExportError(_)
            | RitmoErr::NameParsingError(_)
            | RitmoErr::MergeError(_)
            | RitmoErr::SerializationError(_)
            | RitmoErr::MLError(_)
            | RitmoErr::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(json!({ "error": self.0.to_string() }))).into_response()
    }
}
