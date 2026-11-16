use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use kernel::FieldError;
use serde::Serialize;

/// Corps d'erreur au format RFC 9457 (`application/problem+json`).
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct Problem {
    pub status: u16,
    pub title: String,
    pub detail: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[schema(value_type = Vec<Object>)]
    pub errors: Vec<FieldError>,
}

impl Problem {
    pub fn new(status: StatusCode, detail: impl Into<String>) -> Self {
        let title = status.canonical_reason().unwrap_or("Erreur").to_owned();
        Self {
            status: status.as_u16(),
            title,
            detail: detail.into(),
            errors: Vec::new(),
        }
    }
}

impl IntoResponse for Problem {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let ctype = [(header::CONTENT_TYPE, "application/problem+json")];
        (status, ctype, Json(self)).into_response()
    }
}
