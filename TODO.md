# TODO: Upgrade Test Suite

## Step 1: Upgrade `tests/common/mod.rs` with auth helper and response parsing

All handlers now require `AuthUser` (JWT). Tests need a helper to generate valid tokens, and a helper to parse response bodies.

```rust
use axum::Router;
use axum::body::Body;
use axum::http::Request;
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

/// Generate a valid JWT for testing.
/// Uses the same secret as config (JWT_SECRET env var or "development_secret" default).
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

/// Build an authenticated GET request.
pub fn auth_get(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .header("authorization", format!("Bearer {}", test_token()))
        .body(Body::empty())
        .unwrap()
}

/// Build an authenticated POST request with JSON body.
pub fn auth_post(uri: &str, json: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", test_token()))
        .body(Body::from(json.to_string()))
        .unwrap()
}

/// Build an authenticated PUT request with JSON body.
pub fn auth_put(uri: &str, json: &str) -> Request<Body> {
    Request::builder()
        .method("PUT")
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", test_token()))
        .body(Body::from(json.to_string()))
        .unwrap()
}

/// Build an authenticated DELETE request.
pub fn auth_delete(uri: &str) -> Request<Body> {
    Request::builder()
        .method("DELETE")
        .uri(uri)
        .header("authorization", format!("Bearer {}", test_token()))
        .body(Body::empty())
        .unwrap()
}

/// Parse response body as JSON into the given type.
pub async fn parse_body<T: DeserializeOwned>(response: axum::http::Response<Body>) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).expect("Failed to parse response body")
}
```

Note: you'll need these dev-dependencies in `Cargo.toml`:

```toml
[dev-dependencies]
http-body-util = "0.1"
```

(`jsonwebtoken`, `uuid`, `chrono`, `serde_json`, `tower`, `tokio`, `axum`, `sqlx`, and `dotenvy` are already in your main dependencies and available to tests.)

---

## Step 2: Expand `tests/health_test.rs`

Add tests for liveness and readiness probes, and verify the response body content.

```rust
mod common;

use axum::http::StatusCode;
use axum::{body::Body, http::Request};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_check() {
    let app = common::build_test_app().await;

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

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["status"], "healthy");
    assert!(body["version"].is_string());
}

#[tokio::test]
async fn test_liveness_probe() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health/live")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_readiness_probe() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

---

## Step 3: Rewrite `tests/user_tests.rs`

Replace the file entirely. This covers all CRUD operations, validation, auth, conflict, not-found, and pagination.

```rust
mod common;

use axum::http::StatusCode;
use serde_json::Value;
use tower::ServiceExt;

// ---------------------------------------------------------------------------
// Auth
// ---------------------------------------------------------------------------

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

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ---------------------------------------------------------------------------
// Create
// ---------------------------------------------------------------------------

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
    let app = common::build_test_app().await;

    // Create first user
    let _ = app
        .clone()
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"dupe@example.com","name":"First","password":"password123"}"#,
        ))
        .await
        .unwrap();

    // Attempt duplicate
    let response = app
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"dupe@example.com","name":"Second","password":"password123"}"#,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_user_invalid_email() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"not-an-email","name":"Test","password":"password123"}"#,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_user_short_password() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"test@example.com","name":"Test","password":"short"}"#,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_user_empty_name() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"test@example.com","name":"","password":"password123"}"#,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_user_missing_fields() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_post("/api/users", r#"{"email":"test@example.com"}"#))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_user_invalid_json() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_post("/api/users", r#"not json"#))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ---------------------------------------------------------------------------
// Get
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_user() {
    let app = common::build_test_app().await;

    // Create a user first
    let create_resp = app
        .clone()
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"get@example.com","name":"Get Test","password":"password123"}"#,
        ))
        .await
        .unwrap();

    let created: Value = common::parse_body(create_resp).await;
    let id = created["id"].as_u64().unwrap();

    // Fetch it
    let response = app
        .oneshot(common::auth_get(&format!("/api/users/{}", id)))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["email"], "get@example.com");
    assert_eq!(body["name"], "Get Test");
}

#[tokio::test]
async fn test_get_user_not_found() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_get("/api/users/999999"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_update_user_name() {
    let app = common::build_test_app().await;

    // Create
    let create_resp = app
        .clone()
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"update@example.com","name":"Old Name","password":"password123"}"#,
        ))
        .await
        .unwrap();

    let created: Value = common::parse_body(create_resp).await;
    let id = created["id"].as_u64().unwrap();

    // Update name only
    let response = app
        .oneshot(common::auth_put(
            &format!("/api/users/{}", id),
            r#"{"name":"New Name"}"#,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["name"], "New Name");
    assert_eq!(body["email"], "update@example.com"); // unchanged
}

#[tokio::test]
async fn test_update_user_email() {
    let app = common::build_test_app().await;

    let create_resp = app
        .clone()
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"old@example.com","name":"Test","password":"password123"}"#,
        ))
        .await
        .unwrap();

    let created: Value = common::parse_body(create_resp).await;
    let id = created["id"].as_u64().unwrap();

    let response = app
        .oneshot(common::auth_put(
            &format!("/api/users/{}", id),
            r#"{"email":"new@example.com"}"#,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["email"], "new@example.com");
    assert_eq!(body["name"], "Test"); // unchanged
}

#[tokio::test]
async fn test_update_user_empty_body() {
    let app = common::build_test_app().await;

    let create_resp = app
        .clone()
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"empty@example.com","name":"Test","password":"password123"}"#,
        ))
        .await
        .unwrap();

    let created: Value = common::parse_body(create_resp).await;
    let id = created["id"].as_u64().unwrap();

    // Empty update — should return current user unchanged
    let response = app
        .oneshot(common::auth_put(&format!("/api/users/{}", id), r#"{}"#))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["email"], "empty@example.com");
}

#[tokio::test]
async fn test_update_user_not_found() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_put(
            "/api/users/999999",
            r#"{"name":"Ghost"}"#,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_user_invalid_email() {
    let app = common::build_test_app().await;

    let create_resp = app
        .clone()
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"valid@example.com","name":"Test","password":"password123"}"#,
        ))
        .await
        .unwrap();

    let created: Value = common::parse_body(create_resp).await;
    let id = created["id"].as_u64().unwrap();

    let response = app
        .oneshot(common::auth_put(
            &format!("/api/users/{}", id),
            r#"{"email":"not-valid"}"#,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ---------------------------------------------------------------------------
// Delete
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_delete_user() {
    let app = common::build_test_app().await;

    // Create
    let create_resp = app
        .clone()
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"delete@example.com","name":"Delete Me","password":"password123"}"#,
        ))
        .await
        .unwrap();

    let created: Value = common::parse_body(create_resp).await;
    let id = created["id"].as_u64().unwrap();

    // Delete
    let response = app
        .clone()
        .oneshot(common::auth_delete(&format!("/api/users/{}", id)))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Confirm it's gone
    let get_resp = app
        .oneshot(common::auth_get(&format!("/api/users/{}", id)))
        .await
        .unwrap();

    assert_eq!(get_resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_user_not_found() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_delete("/api/users/999999"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// List / Pagination
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_users_empty() {
    let app = common::build_test_app().await;

    let response = app
        .oneshot(common::auth_get("/api/users"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
    assert_eq!(body["total"], 0);
    assert_eq!(body["page"], 1);
    assert_eq!(body["per_page"], 10);
}

#[tokio::test]
async fn test_list_users_returns_created_users() {
    let app = common::build_test_app().await;

    // Create two users
    let _ = app
        .clone()
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"list1@example.com","name":"User One","password":"password123"}"#,
        ))
        .await
        .unwrap();

    let _ = app
        .clone()
        .oneshot(common::auth_post(
            "/api/users",
            r#"{"email":"list2@example.com","name":"User Two","password":"password123"}"#,
        ))
        .await
        .unwrap();

    let response = app
        .oneshot(common::auth_get("/api/users"))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 2);
    assert_eq!(body["total"], 2);
}

#[tokio::test]
async fn test_list_users_pagination() {
    let app = common::build_test_app().await;

    // Create 3 users
    for i in 1..=3 {
        let _ = app
            .clone()
            .oneshot(common::auth_post(
                "/api/users",
                &format!(
                    r#"{{"email":"page{}@example.com","name":"User {}","password":"password123"}}"#,
                    i, i
                ),
            ))
            .await
            .unwrap();
    }

    // Page 1, per_page=2 — should get 2 users
    let response = app
        .clone()
        .oneshot(common::auth_get("/api/users?page=1&per_page=2"))
        .await
        .unwrap();

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 2);
    assert_eq!(body["total"], 3);
    assert_eq!(body["page"], 1);
    assert_eq!(body["per_page"], 2);

    // Page 2, per_page=2 — should get 1 user
    let response = app
        .oneshot(common::auth_get("/api/users?page=2&per_page=2"))
        .await
        .unwrap();

    let body: Value = common::parse_body(response).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 1);
    assert_eq!(body["total"], 3);
    assert_eq!(body["page"], 2);
}
```

---

## Step 4: Add `http-body-util` as a dev dependency

Add to `Cargo.toml`:

```toml
[dev-dependencies]
http-body-util = "0.1"
```

This is used by `parse_body()` in the test helpers. (`http-body-util` is already in your main deps, but it's good practice to also list it in dev-deps if you want to eventually remove it from main.)

---

## Step 5: Verify

```bash
cargo test         # all tests pass?
cargo test -- --list   # see all test names
```

### Expected test count: 20

**Health (3):**
- `test_health_check`
- `test_liveness_probe`
- `test_readiness_probe`

**Auth (1):**
- `test_unauthenticated_request_returns_401`

**Create (5):**
- `test_create_user`
- `test_create_user_duplicate_email`
- `test_create_user_invalid_email`
- `test_create_user_short_password`
- `test_create_user_empty_name`
- `test_create_user_missing_fields`
- `test_create_user_invalid_json`

**Get (2):**
- `test_get_user`
- `test_get_user_not_found`

**Update (5):**
- `test_update_user_name`
- `test_update_user_email`
- `test_update_user_empty_body`
- `test_update_user_not_found`
- `test_update_user_invalid_email`

**Delete (2):**
- `test_delete_user`
- `test_delete_user_not_found`

**List/Pagination (3):**
- `test_list_users_empty`
- `test_list_users_returns_created_users`
- `test_list_users_pagination`
