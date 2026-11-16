use app::http::ApiError;
use axum::response::IntoResponse;
use http_body_util::BodyExt;
use kernel::{DomainError, FieldError};

async fn render(err: DomainError) -> (u16, String, serde_json::Value) {
    let res = ApiError::from(err).into_response();
    let status = res.status().as_u16();
    let ctype = res.headers()["content-type"].to_str().unwrap().to_owned();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (status, ctype, serde_json::from_slice(&bytes).unwrap())
}

#[tokio::test]
async fn maps_each_domain_error_to_its_status() {
    let cases = [
        (DomainError::not_found("student", "1"), 404),
        (DomainError::Validation(vec![]), 422),
        (DomainError::Conflict("x".into()), 409),
        (DomainError::Unauthorized, 401),
        (DomainError::Forbidden, 403),
        (DomainError::Internal("x".into()), 500),
    ];
    for (err, expected) in cases {
        let (status, ctype, body) = render(err).await;
        assert_eq!(status, expected);
        assert_eq!(ctype, "application/problem+json");
        assert_eq!(body["status"], expected);
    }
}

#[tokio::test]
async fn validation_lists_field_errors() {
    let err = DomainError::Validation(vec![FieldError::new("email", "required", "Obligatoire")]);
    let (_, _, body) = render(err).await;
    assert_eq!(body["errors"][0]["field"], "email");
}

#[tokio::test]
async fn internal_error_never_leaks_details() {
    let (_, _, body) = render(DomainError::Internal("mot de passe db".into())).await;
    assert!(!body.to_string().contains("mot de passe"));
}
