use super::{ApiError, openapi, spa};
use crate::state::AppState;
use axum::{Json, Router, http::StatusCode, routing::get};
use std::path::Path;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use utoipa_scalar::{Scalar, Servable};

pub fn build_router(state: AppState, static_dir: &Path) -> Router {
    let (api, spec) = openapi::api_router().split_for_parts();
    let docs: Router = Scalar::with_url("/api/docs", spec.clone()).into();
    api.with_state(state)
        .route("/api/openapi.json", get(move || async move { Json(spec) }))
        .route("/api/{*rest}", get(api_not_found).fallback(api_not_found))
        .merge(docs)
        .fallback_service(spa::service(static_dir))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
}

async fn api_not_found() -> ApiError {
    ApiError::new(StatusCode::NOT_FOUND, "Route d'API inconnue")
}
