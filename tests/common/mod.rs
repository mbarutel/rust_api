use axum::body::Body;
use axum::http::Request;
use axum::{Router, http::header};
use http_body_util::BodyExt;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::de::DeserializeOwned;
use uuid::Uuid;

pub async fn build_test_app() -> Router {
    dotenvy::dotenv().ok();
    let config = rust_api::config::Config::from_env();
    let state = rust_api::state::AppState::new(&config)
        .await
        .expect("Failed to create test app state");

    sqlx::migrate!()
        .run(&state.db)
        .await
        .expect("Failed to run migrations");

    sqlx::query("DELETE FROM users")
        .execute(&state.db)
        .await
        .expect("Failed to clean users table");

    rust_api::build_router(state)
}

// Generate a valid JWT for testing.
// Uses the same secret as config (JWT_SECRET env var or "development_secret" default).
pub fn test_token() -> String {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "development_secret".to_string());

    let claims = serde_json::json!({
        "sub": Uuid::new_v4().to_string(),
        "email": "testuser@example.com",
        "iat": chrono::Utc::now().timestamp() as usize,
        "exp": (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    });

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to create test JWT")
}

// Build an authenticated GET request.
pub fn auth_get(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .header("authorization", format!("Bearer {}", test_token()))
        .body(Body::empty())
        .unwrap()
}

// Build an authenticated POST request with JSON body.
pub fn auth_post(uri: &str, json: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", test_token()))
        .body(Body::from(json.to_string()))
        .unwrap()
}

// Build an authenticated PUT request with JSON body.
pub fn auth_put(uri: &str, json: &str) -> Request<Body> {
    Request::builder()
        .method("PUT")
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", test_token()))
        .body(Body::from(json.to_string()))
        .unwrap()
}

// Build an authenticated DELETE request.
pub fn auth_delete(uri: &str) -> Request<Body> {
    Request::builder()
        .method("DELETE")
        .uri(uri)
        .header("authorization", format!("Bearer {}", test_token()))
        .body(Body::empty())
        .unwrap()
}

// Parse response body as JSON into the given type.
pub async fn parse_body<T: DeserializeOwned>(response: axum::http::Response<Body>) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).expect("Failed to parse response body")
}
