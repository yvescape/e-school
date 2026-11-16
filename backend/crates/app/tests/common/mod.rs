#![allow(dead_code)]

mod fixtures;

#[allow(unused_imports)]
pub use fixtures::{offline_router, static_dir};

use axum::{Router, body::Body, http::Request};
use http_body_util::BodyExt;
use tower::ServiceExt;

pub struct TestResponse {
    pub status: u16,
    pub content_type: String,
    pub request_id: Option<String>,
    pub body: String,
}

impl TestResponse {
    pub fn json(&self) -> serde_json::Value {
        serde_json::from_str(&self.body).expect("JSON body")
    }
}

pub async fn get(router: Router, uri: &str) -> TestResponse {
    let req = Request::get(uri).body(Body::empty()).unwrap();
    let res = router.oneshot(req).await.unwrap();
    let header = |name: &str| {
        res.headers()
            .get(name)
            .map(|v| v.to_str().unwrap().to_owned())
    };
    let content_type = header("content-type").unwrap_or_default();
    let request_id = header("x-request-id");
    let status = res.status().as_u16();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    TestResponse {
        status,
        content_type,
        request_id,
        body: String::from_utf8_lossy(&bytes).into(),
    }
}
