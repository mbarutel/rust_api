# Migrate to Onion Architecture

Based on: [Rust, Axum and Onion Architecture](https://medium.com/@jonathan.el.baz/rust-axum-and-onion-architecture-escaping-the-tech-debt-spiral-14df5db946df)

## Current Structure (feature-based)

```
src/
  auth/        (handler, service, model, routes)
  users/       (handler, service, model, repository, routes)
  health/
  common/
  middleware/
  config.rs, state.rs, error.rs, lib.rs, main.rs
```

## Target Structure (onion layers)

```
src/
  domain/
    mod.rs
    user.rs              # User model (pure struct, no framework deps)
    error.rs             # DomainError enum
    repository/
      mod.rs
      user_repository.rs # UserRepository trait
  application/
    mod.rs
    error.rs             # AppError enum (wraps DomainError)
    dto/
      mod.rs
      user_dto.rs        # CreateUserDto, UpdateUserDto, UserResponse
      auth_dto.rs        # LoginRequest, RegisterRequest, TokenResponse
    service/
      mod.rs
      user_service.rs    # UserService trait
      auth_service.rs    # AuthService trait
  infrastructure/
    mod.rs
    config.rs            # Config (env loading)
    database/
      mod.rs
      pool.rs            # MySqlPool setup
      entity/
        mod.rs
        user_entity.rs   # DB row struct + From<User>/Into<User> conversions
      repository/
        mod.rs
        user_repository.rs  # impl UserRepository for DbUserRepository
    service/
      mod.rs
      user_service.rs    # impl UserService (holds repo + deps)
      auth_service.rs    # impl AuthService (holds repo + config)
    password.rs          # argon2 hash/verify (infra concern)
  presentation/
    mod.rs
    error.rs             # HandlerError + impl IntoResponse
    middleware/
      mod.rs
      auth.rs            # AuthUser JWT extractor
      validated_json.rs  # ValidateJson extractor
      rate_limiting.rs   # Rate limiter config
    handler/
      mod.rs
      user_handler.rs    # Axum handlers for /api/users
      auth_handler.rs    # Axum handlers for /api/auth
      health_handler.rs  # Health check handlers
    routes.rs            # All route definitions
  state.rs               # AppState (wires Arc<dyn Trait> for DI)
  lib.rs                 # build_router()
  main.rs                # Composition root
```

---

## Phase 1 — Domain Layer (innermost, zero dependencies)

The domain layer contains pure business types and trait definitions. It must not depend on
axum, sqlx, serde, or any framework crate.

### Step 1.1 — Create domain models

Extract `User` from the current `users/model.rs` into a pure struct with no derive macros
except `Debug`, `Clone`.

```rust
// src/domain/user.rs

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
```

`chrono` is acceptable here — it's a data type library, not a framework.

### Step 1.2 — Define repository traits

Move the repository contract to the domain layer. These are the interfaces that the
infrastructure layer will implement.

```rust
// src/domain/repository/user_repository.rs

use crate::domain::user::User;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: u64) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<User>, DomainError>;
    async fn count(&self) -> Result<i64, DomainError>;
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn update(&self, user: User) -> Result<User, DomainError>;
    async fn delete(&self, id: u64) -> Result<(), DomainError>;
}
```

### Step 1.3 — Define domain errors

```rust
// src/domain/error.rs

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("entity not found")]
    NotFound,
    #[error("entity already exists")]
    Conflict,
    #[error("database error: {0}")]
    Database(String),
}
```

`thiserror` is acceptable — it's a derive macro helper, not a framework dependency.

### Files to create

| File | Contents |
|------|----------|
| `src/domain/mod.rs` | `pub mod user; pub mod error; pub mod repository;` |
| `src/domain/user.rs` | Pure `User` struct |
| `src/domain/error.rs` | `DomainError` enum |
| `src/domain/repository/mod.rs` | `pub mod user_repository;` |
| `src/domain/repository/user_repository.rs` | `UserRepository` trait |

---

## Phase 2 — Application Layer (orchestration, DTOs, service traits)

Depends only on the domain layer. Defines service interfaces and data transfer objects.

### Step 2.1 — Define service traits

Move `UserService` and `AuthService` trait definitions here. The traits reference DTOs
(not domain models) for input, and return domain models or DTOs as appropriate.

```rust
// src/application/service/user_service.rs

use crate::application::dto::user_dto::{CreateUserDto, UpdateUserDto};
use crate::application::error::AppError;
use crate::domain::user::User;

#[async_trait::async_trait]
pub trait UserService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<User>, i64), AppError>;
    async fn get(&self, id: u64) -> Result<User, AppError>;
    async fn create(&self, dto: CreateUserDto) -> Result<User, AppError>;
    async fn update(&self, id: u64, dto: UpdateUserDto) -> Result<User, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
}
```

```rust
// src/application/service/auth_service.rs

use crate::application::dto::auth_dto::{LoginRequest, RegisterRequest};
use crate::application::error::AppError;

#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, request: LoginRequest) -> Result<String, AppError>;
    async fn register(&self, request: RegisterRequest) -> Result<String, AppError>;
}
```

### Step 2.2 — Define DTOs

Move request/response structs from `users/model.rs` and `auth/model.rs` here.
These can use `serde` and `validator` — they are serialization boundary types.

```rust
// src/application/dto/user_dto.rs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 1))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserResponse { /* strip password_hash, format timestamps */ }
```

### Step 2.3 — Define application errors

```rust
// src/application/error.rs

use crate::domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("internal error: {0}")]
    Internal(String),
}
```

### Files to create

| File | Contents |
|------|----------|
| `src/application/mod.rs` | `pub mod service; pub mod dto; pub mod error;` |
| `src/application/service/mod.rs` | `pub mod user_service; pub mod auth_service;` |
| `src/application/service/user_service.rs` | `UserService` trait |
| `src/application/service/auth_service.rs` | `AuthService` trait |
| `src/application/dto/mod.rs` | `pub mod user_dto; pub mod auth_dto;` |
| `src/application/dto/user_dto.rs` | Request/response DTOs + `From<User>` |
| `src/application/dto/auth_dto.rs` | Login/register DTOs + token response |
| `src/application/error.rs` | `AppError` wrapping `DomainError` |

---

## Phase 3 — Infrastructure Layer (concrete implementations)

This is where sqlx, argon2, and other external crates live. Implements the traits
defined in the domain and application layers.

### Step 3.1 — Create database entity structs

These mirror the DB schema and convert to/from domain models.

```rust
// src/infrastructure/database/entity/user_entity.rs

#[derive(Debug, sqlx::FromRow)]
pub struct UserEntity {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<UserEntity> for User {
    fn from(e: UserEntity) -> Self {
        User { id: e.id, email: e.email, name: e.name,
               password_hash: e.password_hash,
               created_at: e.created_at, updated_at: e.updated_at }
    }
}
```

### Step 3.2 — Implement repository traits

Move query logic from `users/repository.rs` here.

```rust
// src/infrastructure/database/repository/user_repository.rs

pub struct DbUserRepository {
    pool: MySqlPool,
}

#[async_trait::async_trait]
impl UserRepository for DbUserRepository {
    async fn find_by_id(&self, id: u64) -> Result<Option<User>, DomainError> {
        let entity = sqlx::query_as!(UserEntity, "SELECT * FROM users WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Database(e.to_string()))?;
        Ok(entity.map(User::from))
    }
    // ... other methods
}
```

### Step 3.3 — Implement service traits

Move business logic from `users/service.rs` and `auth/service.rs` here. Services are
generic over their repository dependencies (static dispatch internally, dynamic dispatch
at the composition root).

```rust
// src/infrastructure/service/user_service.rs

pub struct UserServiceImpl {
    repo: Arc<dyn UserRepository>,
}

#[async_trait::async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, dto: CreateUserDto) -> Result<User, AppError> {
        let password_hash = hash_password(&dto.password)
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let user = User {
            id: 0, // auto-assigned by DB
            email: dto.email,
            name: dto.name,
            password_hash,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        self.repo.create(user).await.map_err(AppError::from)
    }
    // ...
}
```

### Step 3.4 — Move config and password utilities

- `src/config.rs` → `src/infrastructure/config.rs`
- `src/common/password.rs` → `src/infrastructure/password.rs`

### Files to create/move

| File | Contents |
|------|----------|
| `src/infrastructure/mod.rs` | `pub mod database; pub mod service; pub mod config; pub mod password;` |
| `src/infrastructure/database/mod.rs` | `pub mod pool; pub mod entity; pub mod repository;` |
| `src/infrastructure/database/pool.rs` | Pool creation (from current `state.rs`) |
| `src/infrastructure/database/entity/mod.rs` | `pub mod user_entity;` |
| `src/infrastructure/database/entity/user_entity.rs` | `UserEntity` + conversions |
| `src/infrastructure/database/repository/mod.rs` | `pub mod user_repository;` |
| `src/infrastructure/database/repository/user_repository.rs` | `DbUserRepository` |
| `src/infrastructure/service/mod.rs` | `pub mod user_service; pub mod auth_service;` |
| `src/infrastructure/service/user_service.rs` | `UserServiceImpl` |
| `src/infrastructure/service/auth_service.rs` | `AuthServiceImpl` |
| `src/infrastructure/config.rs` | Moved from `src/config.rs` |
| `src/infrastructure/password.rs` | Moved from `src/common/password.rs` |

---

## Phase 4 — Presentation Layer (HTTP-facing)

Axum handlers, routes, middleware, and HTTP error responses. Depends on the application
layer only — never imports from infrastructure.

### Step 4.1 — Move handlers

Move handlers from `users/handler.rs`, `auth/handler.rs`, `health/mod.rs` into
`presentation/handler/`. Handlers call service traits (from the application layer)
via `State<AppState>`.

```rust
// src/presentation/handler/user_handler.rs

use crate::application::service::user_service::UserService;
use crate::application::dto::user_dto::{CreateUserDto, UserResponse};

pub async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateUserDto>,
) -> Result<Json<UserResponse>, HandlerError> {
    let user = state.user_service.create(dto).await?;
    Ok(Json(UserResponse::from(user)))
}
```

### Step 4.2 — Create presentation error type

```rust
// src/presentation/error.rs

use crate::application::error::AppError;

#[derive(Debug)]
pub struct HandlerError(AppError);

impl From<AppError> for HandlerError {
    fn from(e: AppError) -> Self { HandlerError(e) }
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            AppError::Domain(DomainError::NotFound) => (StatusCode::NOT_FOUND, "not found"),
            AppError::Domain(DomainError::Conflict) => (StatusCode::CONFLICT, "already exists"),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.as_str()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

### Step 4.3 — Move middleware

- `src/middleware/auth.rs` → `src/presentation/middleware/auth.rs`
- `src/middleware/validated_json.rs` → `src/presentation/middleware/validated_json.rs`
- `src/middleware/rate_limiting.rs` → `src/presentation/middleware/rate_limiting.rs`

### Step 4.4 — Consolidate routes

```rust
// src/presentation/routes.rs

pub fn routes(state: AppState) -> Router {
    Router::new()
        .merge(health_routes())
        .merge(user_routes())
        .merge(auth_routes())
        .with_state(state)
}
```

### Files to create/move

| File | Contents |
|------|----------|
| `src/presentation/mod.rs` | `pub mod handler; pub mod middleware; pub mod routes; pub mod error;` |
| `src/presentation/handler/mod.rs` | `pub mod user_handler; pub mod auth_handler; pub mod health_handler;` |
| `src/presentation/handler/user_handler.rs` | Moved from `users/handler.rs` |
| `src/presentation/handler/auth_handler.rs` | Moved from `auth/handler.rs` |
| `src/presentation/handler/health_handler.rs` | Moved from `health/mod.rs` |
| `src/presentation/error.rs` | `HandlerError` + `IntoResponse` |
| `src/presentation/middleware/mod.rs` | `pub mod auth; pub mod validated_json; pub mod rate_limiting;` |
| `src/presentation/middleware/*.rs` | Moved from `src/middleware/` |
| `src/presentation/routes.rs` | All route definitions |

---

## Phase 5 — Wire Everything in main.rs (Composition Root)

`main.rs` is the composition root — the only place that knows about all layers.
It constructs concrete implementations and injects them as `Arc<dyn Trait>`.

```rust
// src/main.rs (simplified)

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env();
    let pool = create_pool(&config.database_url).await?;

    // Infrastructure: concrete implementations
    let user_repo = Arc::new(DbUserRepository::new(pool.clone()));
    let user_service = Arc::new(UserServiceImpl::new(user_repo.clone()));
    let auth_service = Arc::new(AuthServiceImpl::new(user_repo.clone(), config.clone()));

    // AppState holds Arc<dyn Trait> — dynamic dispatch at the boundary
    let state = AppState {
        config: Arc::new(config.clone()),
        user_service: user_service as Arc<dyn UserService>,
        auth_service: auth_service as Arc<dyn AuthService>,
        db: Some(pool),
    };

    let app = build_router(state, &config);
    // ... start server
}
```

---

## Phase 6 — Cleanup

### Step 6.1 — Delete old module directories

Remove after all code has been moved and tests pass:

- `src/auth/`
- `src/users/`
- `src/health/`
- `src/common/`
- `src/middleware/`
- `src/config.rs`
- `src/error.rs`

### Step 6.2 — Update `lib.rs` module declarations

```rust
// src/lib.rs

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod state;
```

### Step 6.3 — Update integration tests

Tests in `tests/` will need import path updates. The `build_test_app()` helper should
construct `AppState` with mock services the same way `main.rs` does.

### Step 6.4 — Verify the dependency rule

Check that imports follow the onion rule (inner layers never import outer layers):

- `domain/` imports nothing from `application/`, `infrastructure/`, or `presentation/`
- `application/` imports only from `domain/`
- `infrastructure/` imports from `domain/` and `application/`
- `presentation/` imports from `application/` (and `domain/` types if needed)
- Only `main.rs` / `lib.rs` / `state.rs` import from all layers

---

## Migration Strategy

Work **inside-out** — build each layer, compile, then move to the next:

1. **Domain** — create `src/domain/` with models, traits, errors. Compile.
2. **Application** — create `src/application/` with DTOs, service traits, errors. Compile.
3. **Infrastructure** — create `src/infrastructure/` with impls. Compile.
4. **Presentation** — create `src/presentation/` with handlers, routes, middleware. Compile.
5. **Wiring** — update `main.rs`, `lib.rs`, `state.rs`. Compile + run tests.
6. **Cleanup** — delete old directories once all tests pass.

At each phase, keep the old code in place and build the new modules alongside it.
Only delete old files in Phase 6 when everything compiles and tests pass.
