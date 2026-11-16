use super::Problem;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use kernel::DomainError;

/// Erreur renvoyée par les handlers : traduit une erreur métier en réponse HTTP.
#[derive(Debug)]
pub struct ApiError(Problem);

impl ApiError {
    pub fn new(status: StatusCode, detail: impl Into<String>) -> Self {
        Self(Problem::new(status, detail))
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        let detail = err.to_string();
        match err {
            DomainError::NotFound { .. } => Self::new(StatusCode::NOT_FOUND, detail),
            DomainError::Conflict(_) => Self::new(StatusCode::CONFLICT, detail),
            DomainError::Unauthorized => Self::new(StatusCode::UNAUTHORIZED, detail),
            DomainError::Forbidden => Self::new(StatusCode::FORBIDDEN, detail),
            DomainError::Validation(fields) => {
                let mut problem = Problem::new(StatusCode::UNPROCESSABLE_ENTITY, detail);
                problem.errors = fields;
                Self(problem)
            }
            DomainError::Internal(cause) => {
                tracing::error!(%cause, "erreur interne");
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Une erreur interne est survenue",
                )
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}
