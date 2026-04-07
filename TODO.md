# Split AppState into Sub-States (Pattern 2)

## Problem

`AppState` is a monolithic struct. Adding `auth_service` forces every test that constructs
`AppState` (e.g. user handler tests) to also provide an `AuthService`, even when irrelevant.

## Goal

Each route group gets its own state type containing only what it needs. The `AuthUser`
extractor works with any state that provides JWT config via a trait bound.

---

## Step 1 — Define a `HasConfig` trait

Create a trait that any state can implement to provide access to config (needed by `AuthUser`).

```rust
// src/state.rs

pub trait HasConfig: Send + Sync + Clone {
    fn config(&self) -> &Config;
}
```

---

## Step 2 — Define per-domain state structs

Replace the monolithic `AppState` with focused state types. Keep `AppState` only for
top-level wiring (constructing everything + holding the db pool).

```rust
// src/state.rs

#[derive(Clone)]
pub struct UserState {
    pub config: Arc<Config>,
    pub user_service: Arc<dyn UserService>,
}

#[derive(Clone)]
pub struct AuthState {
    pub config: Arc<Config>,
    pub auth_service: Arc<dyn AuthService>,
}

#[derive(Clone)]
pub struct HealthState {
    pub db: Option<MySqlPool>,
}
```

Implement `HasConfig` for each state that needs JWT auth:

```rust
impl HasConfig for UserState {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl HasConfig for AuthState {
    fn config(&self) -> &Config {
        &self.config
    }
}
```

Keep the `AppState` struct and `AppState::new()` as a **builder only** — it creates the
db pool and services, then hands out sub-states. It is not used as router state.

```rust
impl AppState {
    pub async fn new(config: &Config) -> anyhow::Result<Self> { /* unchanged */ }

    pub fn user_state(&self) -> UserState {
        UserState {
            config: self.config.clone(),
            user_service: self.user_service.clone(),
        }
    }

    pub fn auth_state(&self) -> AuthState {
        AuthState {
            config: self.config.clone(),
            auth_service: self.auth_service.clone(),
        }
    }

    pub fn health_state(&self) -> HealthState {
        HealthState {
            db: self.db.clone(),
        }
    }
}
```

---

## Step 3 — Make `AuthUser` generic over `HasConfig`

Change `FromRequestParts<AppState>` to `FromRequestParts<S>` with a `HasConfig` bound.

```rust
// src/middleware/auth.rs

impl<S> FromRequestParts<S> for AuthUser
where
    S: HasConfig + Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(state.config().jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        Ok(AuthUser {
            user_id: token_data.claims.sub,
            email: token_data.claims.email,
        })
    }
}
```

---

## Step 4 — Update route files to use sub-states

### `src/users/routes.rs`

```rust
pub fn router() -> Router<UserState> {
    Router::new()
        .route("/api/users", get(handler::list).post(handler::create))
        .route("/api/users/:id", get(handler::get).put(handler::update).delete(handler::delete))
}
```

### `src/users/handler.rs`

Replace `State(state): State<AppState>` with `State(state): State<UserState>` in every
handler. Access the service directly: `state.user_service.create(...)`.

### `src/auth/routes.rs`

```rust
pub fn router() -> Router<AuthState> {
    Router::new()
        .route("/api/auth/login", post(handler::login))
        .route("/api/auth/register", post(handler::register))
}
```

### `src/auth/handler.rs`

Replace `State(state): State<AppState>` with `State(state): State<AuthState>`.

### `src/health/mod.rs`

Replace `State(state): State<AppState>` with `State(state): State<HealthState>`.
The `health` and `liveness` handlers don't use state at all, only `readiness` does.

---

## Step 5 — Wire sub-states in `build_router`

```rust
// src/lib.rs

pub fn build_router(state: AppState, config: &Config) -> Router {
    let router = Router::new()
        .merge(health::router().with_state(state.health_state()))
        .merge(users::routes::router().with_state(state.user_state()))
        .merge(auth::routes::router().with_state(state.auth_state()));

    // layers remain the same — they don't depend on state
    // ...
    router
}
```

Note: since each sub-router now has `.with_state()` applied, they become `Router<()>` after
merging. The final router no longer needs `.with_state(state)` at the bottom.

Remove the `.with_state(state)` call that currently sits at the end of `build_router`.

---

## Step 6 — Fix the tests

### User handler tests (`src/users/handler.rs`)

The `test_app` function simplifies — no `auth_service` needed:

```rust
fn test_app(service: MockUserService) -> Router {
    let config = crate::test_helpers::test_config();
    let state = UserState {
        config: Arc::new(config),
        user_service: Arc::new(service),
    };

    crate::users::routes::router().with_state(state)
}
```

### Auth handler tests (when you add them)

Same idea — only construct `AuthState` with a mock `AuthService`.

### Health tests (if any)

Only need `HealthState { db: None }`.

---

## File change summary

| File                       | Change                                                      |
| -------------------------- | ----------------------------------------------------------- |
| `src/state.rs`             | Add `HasConfig` trait, `UserState`, `AuthState`, `HealthState`, builder methods |
| `src/middleware/auth.rs`   | Make `AuthUser` generic over `S: HasConfig`                 |
| `src/users/routes.rs`      | `Router<AppState>` → `Router<UserState>`                    |
| `src/users/handler.rs`     | `State<AppState>` → `State<UserState>` in all handlers + tests |
| `src/auth/routes.rs`       | `Router<AppState>` → `Router<AuthState>`                    |
| `src/auth/handler.rs`      | `State<AppState>` → `State<AuthState>`                      |
| `src/health/mod.rs`        | `Router<AppState>` → `Router<HealthState>`, update readiness |
| `src/lib.rs`               | Apply `.with_state()` per sub-router, remove final `.with_state(state)` |
