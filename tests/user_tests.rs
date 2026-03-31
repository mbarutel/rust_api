mod common;

use axum::http::StatusCode;
use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[tokio::test]
async fn test_create_user() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"email":"test@example.com","name":"Test","password":"password123"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_delete_user() {
    todo!()
}

#[tokio::test]
async fn test_update_user() {
    todo!()
}

#[tokio::test]
async fn test_validation_error() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"email":"invalid","name":"","password":"short"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
