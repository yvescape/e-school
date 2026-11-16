use super::{ApiError, Problem};
use crate::state::AppState;
use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(Serialize, utoipa::ToSchema)]
pub struct Health {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<String>,
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(live))
        .routes(routes!(ready))
}

/// Le processus répond (sonde de vie).
#[utoipa::path(get, path = "/live", tag = "health",
    responses((status = 200, body = Health)))]
async fn live() -> Json<Health> {
    Json(Health {
        status: "ok".into(),
        database: None,
    })
}

/// L'application peut servir des requêtes : la base répond.
#[utoipa::path(get, path = "/ready", tag = "health",
    responses((status = 200, body = Health), (status = 503, body = Problem)))]
async fn ready(State(state): State<AppState>) -> Result<Json<Health>, ApiError> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|err| {
            tracing::warn!(%err, "base de données injoignable");
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "Base de données injoignable",
            )
        })?;
    Ok(Json(Health {
        status: "ok".into(),
        database: Some("ok".into()),
    }))
}
