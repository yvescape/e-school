use crate::FieldError;

/// Erreurs métier, indépendantes du transport (HTTP, CLI…).
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{entity} {id} introuvable")]
    NotFound { entity: &'static str, id: String },
    #[error("données invalides")]
    Validation(Vec<FieldError>),
    #[error("conflit : {0}")]
    Conflict(String),
    #[error("authentification requise")]
    Unauthorized,
    #[error("action non autorisée")]
    Forbidden,
    #[error("erreur interne : {0}")]
    Internal(String),
}

impl DomainError {
    pub fn not_found(entity: &'static str, id: impl Into<String>) -> Self {
        Self::NotFound {
            entity,
            id: id.into(),
        }
    }
}
