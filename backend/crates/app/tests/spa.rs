mod common;

use common::{get, offline_router, static_dir};

#[tokio::test]
async fn serves_frontend_index() {
    let dir = static_dir();
    let res = get(offline_router(dir.path()), "/").await;
    assert_eq!(res.status, 200);
    assert!(res.body.contains("e_school"));
}

#[tokio::test]
async fn unknown_frontend_route_falls_back_to_index() {
    let dir = static_dir();
    let res = get(offline_router(dir.path()), "/etudiants/123").await;
    assert_eq!(res.status, 200);
    assert!(res.body.contains("e_school"));
}

#[tokio::test]
async fn unknown_api_route_is_a_json_404() {
    let dir = static_dir();
    let res = get(offline_router(dir.path()), "/api/inconnu").await;
    assert_eq!(res.status, 404);
    assert_eq!(res.content_type, "application/problem+json");
}
