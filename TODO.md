# TODO

## Step 1: Wire `ValidateJson<T>` into handlers

Replace `Json<T>` + manual `.validate()` with the `ValidateJson<T>` extractor in `src/middleware/validated_json.rs`. It handles deserialization and validation in one step.

### `src/users/handler.rs` — `create` handler

Change the signature and remove the manual validate call:

```rust
// BEFORE
use axum::Json;

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    // ...
}

// AFTER
use crate::middleware::validated_json::ValidateJson;

pub async fn create(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    // validation already happened in the extractor
    tracing::info!("Creating new user");
    // ... rest stays the same
}
```

### `src/users/handler.rs` — `update` handler

Same pattern:

```rust
// BEFORE
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;
    // ...
}

// AFTER
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ValidateJson(payload): ValidateJson<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    tracing::info!("Updating user");
    // ... rest stays the same
}
```

You can also remove `use validator::Validate;` from `handler.rs` since it's no longer called directly. Keep `axum::Json` for the response types — only the input extraction changes.

---

## Step 2: Apply auth middleware to protected routes

The `AuthUser` extractor in `src/middleware/auth.rs` is already implemented. You just need to add it as a parameter to handlers that should require authentication.

### Option A: Per-handler (add `AuthUser` param)

For individual protected handlers, add the extractor as a parameter:

```rust
// src/users/handler.rs
use crate::middleware::auth::AuthUser;

#[tracing::instrument(skip(state, _user))]
pub async fn delete(
    State(state): State<AppState>,
    _user: AuthUser,  // request will 401 if no valid JWT
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    tracing::info!("Deleting user");
    // ...
}
```

### Option B: Route-layer (protect a group of routes)

To protect all mutating routes at once, split public and protected routes in `src/users/routes.rs`:

```rust
use axum::{Router, routing::get};

use super::handler;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    let public = Router::new()
        .route("/api/users", get(handler::list))
        .route("/api/users/{id}", get(handler::get));

    let protected = Router::new()
        .route("/api/users", axum::routing::post(handler::create))
        .route(
            "/api/users/{id}",
            axum::routing::put(handler::update).delete(handler::delete),
        );

    public.merge(protected)
}
```

Then in the protected handlers, add `_user: AuthUser` as the first extractor param (before `State`) so Axum runs the auth check.

Pick whichever option fits — Option A is simpler, Option B is cleaner when many routes share the same auth requirement.

---

## Step 3: Implement proper rate limiting or remove the stub

The current `src/middleware/rate_limiting.rs` is a stub that passes all requests through. Either implement it properly or delete it.

### Option A: Implement with `tower_governor`

Add to `Cargo.toml`:

```toml
tower_governor = "0.6"
```

Replace `src/middleware/rate_limiting.rs` with:

```rust
use tower_governor::{GovernorConfigBuilder, GovernorLayer};

pub fn rate_limit_layer() -> GovernorLayer {
    let config = GovernorConfigBuilder::default()
        .per_second(2)          // refill rate
        .burst_size(10)         // max burst
        .finish()
        .expect("Failed to build rate limiter config");

    GovernorLayer { config: config.into() }
}
```

Apply it in `src/lib.rs`:

```rust
use crate::middleware::rate_limiting::rate_limit_layer;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(health::router())
        .merge(users::routes::router())
        .layer(rate_limit_layer())  // add before other layers
        .layer(CompressionLayer::new())
        // ... rest unchanged
}
```

### Option B: Remove the stub

Delete `src/middleware/rate_limiting.rs` and remove `pub mod rate_limiting;` from `src/middleware/mod.rs`.

---

## Step 4: Use `PaginatedResponse<T>` in the list handler

Wire up the `PaginatedResponse<T>` from `src/common/pagination.rs`.

### `src/users/repository.rs` — add a count query

```rust
pub async fn count(pool: &MySqlPool) -> Result<u64> {
    let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(count as u64)
}
```

### `src/users/repository.rs` — add pagination to `find_all`

Change the signature to accept page/per_page and add LIMIT/OFFSET:

```rust
pub async fn find_all(pool: &MySqlPool, page: u32, per_page: u32) -> Result<Vec<UserResponse>> {
    let offset = (page - 1) * per_page;

    sqlx::query_as!(
        UserResponse,
        "SELECT id, name, email, created_at, updated_at FROM users LIMIT ? OFFSET ?",
        per_page,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}
```

### `src/users/handler.rs` — return `PaginatedResponse`

```rust
use crate::common::pagination::PaginatedResponse;

#[tracing::instrument(skip(state))]
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginatedResponse<UserResponse>>> {
    tracing::debug!(page = query.page, per_page = query.per_page, "Listing users");

    let total = repository::count(&state.db).await?;
    let users = repository::find_all(&state.db, query.page, query.per_page).await?;

    Ok(Json(PaginatedResponse {
        data: users,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}
```

---

## Step 5: Add a service layer when business logic grows

Not needed yet — do this when a handler starts doing more than validate + call repository (e.g., sending emails, audit logging, multi-step transactions).

When the time comes, create `src/users/service.rs`:

```rust
use sqlx::MySqlPool;

use crate::error::{AppError, Result};
use super::model::{CreateUserRequest, UserResponse};
use super::repository;

pub async fn create_user(pool: &MySqlPool, payload: CreateUserRequest) -> Result<UserResponse> {
    // 1. Check email uniqueness
    if repository::email_exists(pool, &payload.email).await? {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // 2. Hash password
    let password_hash = hash_password(&payload.password)?;

    // 3. Insert user
    let now = chrono::Utc::now();
    let id = repository::insert(pool, &payload.email, &payload.name, &password_hash, now).await?;

    // 4. Send welcome email (future)
    // email::send_welcome(&payload.email).await?;

    // 5. Create audit log (future)
    // audit::log("user_created", id).await?;

    Ok(UserResponse {
        id,
        email: payload.email,
        name: payload.name,
        created_at: now,
        updated_at: now,
    })
}

fn hash_password(password: &str) -> Result<String> {
    use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};

    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::Error::msg(e.to_string())))
        .map(|h| h.to_string())
}
```

Then the handler becomes:

```rust
pub async fn create(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    let user = service::create_user(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
```

Add `pub mod service;` to `src/users/mod.rs` when you create the file.
