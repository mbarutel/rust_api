# TODO: Test Suite Restructuring

Goal: Prepare the codebase for both unit tests (no DB, fast) and integration tests (real DB). The service layer is the right abstraction boundary for mocking — the repository uses `sqlx::query_as!` macros (compile-time checked, not practical to trait-ify).

---

## Step 1: Complete the Service Layer

Handlers currently bypass the service layer for `get`, `list`, `update`, `delete` — calling `repository::*` directly. All handler logic must go through the service layer first.

**`src/users/service.rs`** — Add these functions (thin wrappers over repository):

```rust
pub async fn list_users(pool: &MySqlPool, page: u32, per_page: u32) -> Result<(Vec<UserResponse>, u64)> {
    let total = repository::count(pool).await?;
    let users = repository::find_all(pool, page, per_page).await?;
    Ok((users, total))
}

pub async fn get_user(pool: &MySqlPool, id: u64) -> Result<UserResponse> {
    repository::find_by_id(pool, id).await
}

pub async fn update_user(pool: &MySqlPool, id: u64, email: Option<String>, name: Option<String>) -> Result<UserResponse> {
    repository::update(pool, id, email, name).await
}

pub async fn delete_user(pool: &MySqlPool, id: u64) -> Result<bool> {
    repository::delete(pool, id).await
}
```

**`src/users/handler.rs`** — Update `list`, `get`, `update`, `delete` to call `service::*` instead of `repository::*`. Remove `use super::repository;`.

This is a pure refactor — no behavior change. All existing integration tests should still pass.

---

## Step 2: Introduce a `UserService` Trait

Edition 2024 supports native async trait methods — no `async-trait` crate needed.

**`src/users/service.rs`** — Define trait and implementation:

```rust
pub trait UserService: Send + Sync {
    async fn create(&self, payload: CreateUserRequest) -> Result<UserResponse>;
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<UserResponse>, u64)>;
    async fn get(&self, id: u64) -> Result<UserResponse>;
    async fn update(&self, id: u64, email: Option<String>, name: Option<String>) -> Result<UserResponse>;
    async fn delete(&self, id: u64) -> Result<bool>;
}

pub struct UserServiceImpl {
    pool: MySqlPool,
}

impl UserServiceImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl UserService for UserServiceImpl {
    async fn create(&self, payload: CreateUserRequest) -> Result<UserResponse> {
        // Move existing create_user body here, using self.pool
    }
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<UserResponse>, u64)> {
        // Move list_users body here
    }
    // ... same for get, update, delete
}
```

**`src/state.rs`** — Add service to AppState:

```rust
use std::sync::Arc;
use crate::users::service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: MySqlPool,
    pub user_service: Arc<dyn UserService>,
}
```

**`src/users/handler.rs`** — Use `state.user_service.*()` instead of `service::*(&state.db, ...)`:

```rust
pub async fn create(
    State(state): State<AppState>,
    _users: AuthUser,
    ValidateJson(payload): ValidateJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    let user = state.user_service.create(payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
```

**`src/main.rs`** — Construct `UserServiceImpl` when building state.

**`tests/common/mod.rs`** — Construct `UserServiceImpl` in `build_test_app()`.

---

## Step 3: Add Unit Tests (inline `#[cfg(test)]` modules)

### 3a. Model validation — `src/users/model.rs`

Quick win, no refactoring needed. Pure synchronous tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_create_request() {
        let req = CreateUserRequest {
            email: "test@example.com".into(),
            name: "Test".into(),
            password: "password123".into(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn invalid_email_rejected() {
        let req = CreateUserRequest {
            email: "not-an-email".into(),
            name: "Test".into(),
            password: "password123".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn short_password_rejected() {
        let req = CreateUserRequest {
            email: "test@example.com".into(),
            name: "Test".into(),
            password: "short".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn empty_name_rejected() {
        let req = CreateUserRequest {
            email: "test@example.com".into(),
            name: "".into(),
            password: "password123".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_invalid_email_rejected() {
        let req = UpdateUserRequest {
            email: Some("not-valid".into()),
            name: None,
        };
        assert!(req.validate().is_err());
    }
}
```

### 3b. Handler unit tests — `src/users/handler.rs`

Build a `MockUserService` implementing the trait, inject into `AppState`, use `oneshot()`. No DB needed:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use axum::{body::Body, http::Request, Router};
    use tower::ServiceExt;

    struct MockUserService {
        // Control return values per-method
    }

    impl UserService for MockUserService {
        async fn create(&self, _payload: CreateUserRequest) -> Result<UserResponse> {
            // Return controlled result
        }
        // ... other methods
    }

    fn test_app(service: MockUserService) -> Router {
        let state = AppState {
            config: Arc::new(Config::from_env()),
            db: /* unused, can be a dummy */,
            user_service: Arc::new(service),
        };
        crate::build_router(state)
    }

    #[tokio::test]
    async fn create_returns_201_on_success() { ... }

    #[tokio::test]
    async fn create_returns_409_on_conflict() { ... }

    #[tokio::test]
    async fn get_returns_404_when_not_found() { ... }

    #[tokio::test]
    async fn delete_returns_204_on_success() { ... }
}
```

### 3c. Auth middleware — `src/middleware/auth.rs`

Extract JWT decode logic into a testable helper function, then test:
- Valid token decodes correctly
- Expired token rejected
- Malformed token rejected

---

## Step 4: Rename Integration Test Files (optional clarity)

- `tests/health_test.rs` -> `tests/health_integration.rs`
- `tests/user_tests.rs` -> `tests/user_integration.rs`

---

## Step 5: Add Cargo Aliases

**`.cargo/config.toml`:**

```toml
[alias]
unit = "test --lib"
integration = "test --test '*'"
```

---

## Running Tests

| Command              | What it runs               | DB required? |
|----------------------|----------------------------|--------------|
| `cargo unit`         | `#[cfg(test)]` modules     | No           |
| `cargo integration`  | `tests/*.rs` files         | Yes          |
| `cargo test`         | Everything                 | Yes          |

---

## Implementation Order

1. **Step 1** — Complete service layer (pure refactor, tests still pass)
2. **Step 3a** — Add model validation unit tests (quick win, independent)
3. **Step 2** — Introduce `UserService` trait + update AppState/handlers
4. **Step 3b** — Add handler unit tests with mock service
5. **Step 3c** — Add auth unit tests
6. **Step 4 & 5** — Rename files, add aliases

Each step leaves the project in a compiling, working state.

---

## What NOT to do

- **Don't trait-ify the repository layer** — `query_as!` macros return concrete types, wrapping in a trait defeats compile-time SQL checking
- **Don't add `async-trait` crate** — edition 2024 supports native async methods in traits
- **Don't restructure `tests/` into subdirectories yet** — with only 2 test files the flat structure is fine. Revisit at 5+ modules
- **Don't create separate `src/test_helpers.rs` yet** — start with inline `#[cfg(test)]`, extract when duplication appears across 3+ modules
