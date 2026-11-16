use super::health;
use crate::state::AppState;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(OpenApi)]
#[openapi(info(
    title = "e_school API",
    description = "API de gestion d'établissements"
))]
struct ApiDoc;

/// Toutes les routes documentées de l'API, avec leurs chemins complets.
pub fn api_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/api/health", health::router())
}

/// Document OpenAPI seul (utilisé pour générer le client du frontend).
pub fn spec() -> utoipa::openapi::OpenApi {
    api_router().split_for_parts().1
}
