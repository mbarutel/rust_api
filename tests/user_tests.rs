mod common;

use axum::http::{StatusCode, response};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_unauthenticated_request_returns_401() {
    use axum::{body::Body, http::Request};

    let app = common::build_test_app().await;

    // Request with no Authorization header
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED)
}

#[tokio::test]
async fn test_create_user() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"test@example.com","name":"Test User","password":"password123"}"#,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["email"], "test@example.com");
    assert_eq!(body["name"], "Test User");
    assert!(body["id"].is_number());
    assert!(body["created_at"].is_string());
    assert!(body["updated_at"].is_string());
}

#[tokio::test]
async fn test_create_user_duplicate_email() {
    todo!()
}

#[tokio::test]
async fn test_create_user_invalid_email() {
    todo!()
}

#[tokio::test]
async fn test_create_user_short_password() {
    todo!()
}

#[tokio::test]
async fn test_create_user_empty_name() {
    todo!()
}

#[tokio::test]
async fn test_create_user_missing_fields() {
    todo!()
}

#[tokio::test]
async fn test_create_user_invalid_json() {
    todo!()
}

#[tokio::test]
async fn test_get_user() {
    todo!()
}

#[tokio::test]
async fn test_get_user_not_found() {
    todo!()
}

#[tokio::test]
async fn test_update_user_name() {
    todo!()
}

#[tokio::test]
async fn test_update_user_email() {
    todo!()
}

#[tokio::test]
async fn test_update_user_empty_body() {
    todo!()
}

#[tokio::test]
async fn test_update_user_not_found() {
    todo!()
}

#[tokio::test]
async fn test_update_user_invalid_email() {
    todo!()
}

#[tokio::test]
async fn test_delete_user() {
    todo!()
}

#[tokio::test]
async fn test_delete_user_not_found() {
    todo!()
}

#[tokio::test]
async fn test_list_users_empty() {
    todo!()
}

#[tokio::test]
async fn test_list_users_returns_created_users() {
    todo!()
}

#[tokio::test]
async fn test_list_users_pagination() {
    todo!()
}
