use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

#[tokio::test]
async fn test_health_check() {
    let app = build_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user() {
    let app = build_test_app().await;

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
async fn test_validation_error() {
    let app = build_test_app().await;

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

async fn build_test_app() -> axum::Router {
    dotenvy::dotenv().ok();
    let config = rust_api::config::Config::from_env();
    let state = rust_api::state::AppState::new(&config)
        .await
        .expect("Failed to create test app state");

    // Run Migrations
    sqlx::migrate!()
        .run(&state.db)
        .await
        .expect("Failed to run migrations");

    // Clean slate for each test
    sqlx::query("DELETE FROM users")
        .execute(&state.db)
        .await
        .expect("Failed to clean users table");

    rust_api::build_router(state)
}
