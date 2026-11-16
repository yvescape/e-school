mod common;

use common::{get, offline_router, static_dir};

#[tokio::test]
async fn liveness_answers_without_database() {
    let dir = static_dir();
    let res = get(offline_router(dir.path()), "/api/health/live").await;
    assert_eq!(res.status, 200);
    assert_eq!(res.json(), serde_json::json!({"status": "ok"}));
}

#[tokio::test]
async fn readiness_fails_when_database_is_down() {
    let dir = static_dir();
    let res = get(offline_router(dir.path()), "/api/health/ready").await;
    assert_eq!(res.status, 503);
    assert_eq!(res.content_type, "application/problem+json");
}

#[tokio::test]
async fn every_response_carries_a_request_id() {
    let dir = static_dir();
    let res = get(offline_router(dir.path()), "/api/health/live").await;
    assert!(res.request_id.is_some_and(|id| !id.is_empty()));
}
