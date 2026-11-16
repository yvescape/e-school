mod common;

use common::{get, offline_router, static_dir};

#[tokio::test]
async fn publishes_openapi_document() {
    let dir = static_dir();
    let res = get(offline_router(dir.path()), "/api/openapi.json").await;
    assert_eq!(res.status, 200);
    let paths = &res.json()["paths"];
    assert!(paths.get("/api/health/live").is_some());
    assert!(paths.get("/api/health/ready").is_some());
}

#[tokio::test]
async fn serves_interactive_api_docs() {
    let dir = static_dir();
    let res = get(offline_router(dir.path()), "/api/docs").await;
    assert_eq!(res.status, 200);
    assert!(res.content_type.starts_with("text/html"));
}
