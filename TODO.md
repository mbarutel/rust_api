# Restructure: Feature-Based Module Organization

## Target Structure

```
src/
  main.rs                    // no changes
  lib.rs                     // updated module declarations + router
  config.rs                  // no changes
  state.rs                   // no changes
  error.rs                   // no changes
  common/
    mod.rs
    pagination.rs            // generic PaginatedResponse<T>
  health/
    mod.rs                   // routes + handlers (small enough for one file)
  users/
    mod.rs                   // re-exports
    model.rs                 // all user types
    repository.rs            // all SQL queries
    handler.rs               // thin handlers
    routes.rs                // user routes
  middleware/
    mod.rs                   // no changes
    auth.rs                  // no changes
    validated_json.rs        // no changes
    rate_limiting.rs         // no changes
tests/
  common/mod.rs              // shared test helpers
  health_tests.rs
  user_tests.rs
```

---

## Step 1: Create `src/users/model.rs`

Move all types out of `handlers/users.rs` into this new file.

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ListQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}
fn default_per_page() -> u32 {
    10
}
```

---

## Step 2: Create `src/users/repository.rs`

Extract all SQL queries from `handlers/users.rs` into standalone functions.

```rust
use sqlx::MySqlPool;

use crate::error::{AppError, Result};
use super::model::UserResponse;

pub async fn find_all(pool: &MySqlPool) -> Result<Vec<UserResponse>> {
    sqlx::query_as!(
        UserResponse,
        "SELECT id, name, email, created_at, updated_at FROM users",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<UserResponse> {
    sqlx::query_as!(
        UserResponse,
        "SELECT id, email, name, created_at, updated_at FROM users WHERE id = ?",
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::Internal(e.into()),
    })
}

pub async fn email_exists(pool: &MySqlPool, email: &str) -> Result<bool> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)",
        email,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(exists == 1)
}

pub async fn insert(
    pool: &MySqlPool,
    email: &str,
    name: &str,
    password_hash: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Result<u64> {
    let result = sqlx::query!(
        "INSERT INTO users (email, name, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        email,
        name,
        password_hash,
        now,
        now,
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(result.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    email: Option<String>,
    name: Option<String>,
) -> Result<UserResponse> {
    let mut set_clauses = Vec::new();
    let mut bindings = Vec::new();

    if let Some(name) = name {
        set_clauses.push("name = ?");
        bindings.push(name);
    }

    if let Some(email) = email {
        set_clauses.push("email = ?");
        bindings.push(email);
    }

    if set_clauses.is_empty() {
        return find_by_id(pool, id).await;
    }

    let sql = format!("UPDATE users SET {} WHERE id = ?", set_clauses.join(", "));

    let mut query = sqlx::query(&sql);
    for binding in &bindings {
        query = query.bind(binding);
    }
    query
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::Internal(e.into()),
        })?;

    find_by_id(pool, id).await
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<bool> {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(result.rows_affected() > 0)
}
```

---

## Step 3: Create `src/users/handler.rs`

Slim handlers that delegate to the repository.

```rust
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use validator::Validate;

use crate::error::{AppError, Result};
use crate::state::AppState;
use super::model::{CreateUserRequest, ListQuery, UpdateUserRequest, UserResponse};
use super::repository;

#[tracing::instrument(skip(state))]
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<UserResponse>>> {
    tracing::debug!(page = query.page, per_page = query.per_page, "Listing users");
    let users = repository::find_all(&state.db).await?;
    Ok(Json(users))
}

#[tracing::instrument(skip(state, payload), fields(user.email = %payload.email))]
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    tracing::info!("Creating new user");

    if repository::email_exists(&state.db, &payload.email).await? {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::Error::msg(e.to_string())))?
        .to_string();

    let now = chrono::Utc::now();
    let id = repository::insert(&state.db, &payload.email, &payload.name, &password_hash, now).await?;

    let user = UserResponse {
        id,
        email: payload.email,
        name: payload.name,
        created_at: now,
        updated_at: now,
    };

    tracing::info!(user.id = %user.id, "User created!");
    Ok((StatusCode::CREATED, Json(user)))
}

#[tracing::instrument(skip(state))]
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<UserResponse>> {
    let user = repository::find_by_id(&state.db, id).await?;
    Ok(Json(user))
}

#[tracing::instrument(skip(state, payload))]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    tracing::info!("Updating user");
    let user = repository::update(&state.db, id, payload.email, payload.name).await?;
    Ok(Json(user))
}

#[tracing::instrument(skip(state))]
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    tracing::info!("Deleting user");

    if !repository::delete(&state.db, id).await? {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
```

---

## Step 4: Create `src/users/routes.rs`

```rust
use axum::{Router, routing::get};

use crate::state::AppState;
use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/users", get(handler::list).post(handler::create))
        .route(
            "/api/users/{id}",
            get(handler::get).put(handler::update).delete(handler::delete),
        )
}
```

---

## Step 5: Create `src/users/mod.rs`

```rust
pub mod handler;
pub mod model;
pub mod repository;
pub mod routes;
```

---

## Step 6: Create `src/health/mod.rs`

Copy `src/routes/health.rs` as-is into this file. Only change the import path.

```rust
use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn liveness() -> &'static str {
    "Ok"
}

async fn readiness(State(state): State<AppState>) -> Result<&'static str, &'static str> {
    if state.db.acquire().await.is_err() {
        return Err("Database Unavailable");
    }
    Ok("Ok")
}
```

---

## Step 7: Create `src/common/pagination.rs`

```rust
use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
}
```

---

## Step 8: Create `src/common/mod.rs`

```rust
pub mod pagination;
```

---

## Step 9: Update `src/lib.rs`

Replace the entire file with:

```rust
pub mod common;
pub mod config;
pub mod error;
pub mod health;
pub mod middleware;
pub mod state;
pub mod users;

use axum::{Router, http::StatusCode};
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(health::router())
        .merge(users::routes::router())
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
                let request_id = request
                    .headers()
                    .get("x-request-id")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown");

                tracing::info_span!(
                    "http_request",
                    method = %request.method(),
                    uri = %request.uri(),
                    request_id = %request_id,
                )
            }),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_methods(Any),
        )
        .with_state(state)
}
```

---

## Step 10: Delete old directories

Remove these files/directories (they've been moved):

- `src/handlers/` (entire directory)
- `src/routes/` (entire directory)
- `src/models/` (entire directory)

---

## Step 11: Split tests

### `tests/common/mod.rs`

```rust
use axum::Router;

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
```

### `tests/health_tests.rs`

```rust
mod common;

use axum::{body::Body, http::Request};
use axum::http::StatusCode;
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
}
```

### `tests/user_tests.rs`

```rust
mod common;

use axum::{body::Body, http::Request};
use axum::http::StatusCode;
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
```

Then delete `tests/api_tests.rs`.

---

## Step 12: Fix Dockerfile

Line 15 copies `web-api` but the package name is `rust-api`, so the binary is `rust-api`. Change:

```dockerfile
COPY --from=builder /app/target/release/rust-api /usr/local/bin/
```

(Line 19 CMD is already correct: `["rust-api"]`)

---

## Step 13: Verify

```bash
cargo check        # compiles?
cargo test         # tests pass?
```

---

## Future TODOs (not part of this restructure)

- [ ] Wire `ValidateJson<T>` extractor into handlers, remove manual `validate()` calls
- [ ] Apply auth middleware to protected routes
- [ ] Implement rate limiting properly (with `governor` crate) or remove the stub
- [ ] Use `PaginatedResponse<T>` in `list` handler (add COUNT query to repository)
- [ ] Add a service layer (`src/users/service.rs`) when business logic outgrows handlers
