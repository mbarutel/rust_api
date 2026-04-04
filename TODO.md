# Auth Module Implementation Guide

Follow the same pattern as the `users` module (`src/users/`).

## New Files

### `src/auth/mod.rs`

- Declare submodules: `model`, `routes`, `handler`, `service`

### `src/auth/model.rs`

Request/response structs with serde + validator derives:

- `LoginRequest` — `email: String` (valid email), `password: String` (min 8 chars)
- `RegisterRequest` — `email: String` (valid email), `name: String` (1-100 chars), `password: String` (min 8 chars)
- `TokenResponse` — `token: String`

### `src/auth/routes.rs`

Two public (unauthenticated) routes:

```rust
Router::new()
    .route("/api/auth/login", post(handler::login))
    .route("/api/auth/register", post(handler::register))
```

No `AuthUser` extractor on these — they produce tokens, not consume them.

### `src/auth/handler.rs`

Two handlers:

- `login(State(state), ValidateJson(payload): ValidateJson<LoginRequest>) -> Result<Json<TokenResponse>>`
  - Calls auth service login, returns token
- `register(State(state), ValidateJson(payload): ValidateJson<RegisterRequest>) -> Result<(StatusCode, Json<TokenResponse>)>`
  - Calls auth service register, returns 201 + token

### `src/auth/service.rs`

Trait + impl, same pattern as `UserService`:

```rust
#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, payload: LoginRequest) -> Result<TokenResponse>;
    async fn register(&self, payload: RegisterRequest) -> Result<TokenResponse>;
}
```

`AuthServiceImpl` holds `MySqlPool` + `Arc<Config>`.

**register:**
1. Check email doesn't exist via `repository::email_exists()`
2. Hash password with Argon2 (same as `UserServiceImpl::create`)
3. Insert user via `repository::insert()`
4. Generate JWT and return `TokenResponse`

**login:**
1. Look up user by email via `repository::find_by_email()`
   - If not found, return `AppError::Unauthorized`
2. Verify password:
   ```rust
   use argon2::{Argon2, PasswordVerifier, PasswordHash};
   let parsed = PasswordHash::new(&user.password_hash).map_err(|_| AppError::Unauthorized)?;
   Argon2::default()
       .verify_password(password.as_bytes(), &parsed)
       .map_err(|_| AppError::Unauthorized)?;
   ```
3. Generate JWT and return `TokenResponse`

**generate_token helper (private):**
- Build `Claims` struct (from `src/middleware/auth.rs`) with:
  - `sub`: user ID (see note below about UUID vs BIGINT)
  - `email`: user's email
  - `iat`: `Utc::now().timestamp() as usize`
  - `exp`: `(Utc::now() + Duration::hours(24)).timestamp() as usize`
- Encode with `jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_bytes()))`

## Changes to Existing Files

### `src/users/repository.rs`

Add a `find_by_email` function that returns a struct including `password_hash`:

```rust
pub struct UserRow {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<UserRow> { ... }
```

Return `AppError::Unauthorized` (not `NotFound`) when the email doesn't exist, to avoid leaking whether an account exists.

### `src/lib.rs`

- Add `pub mod auth;`
- Add `.merge(auth::routes::router())` in `build_router()`

### `src/state.rs`

- Add `pub auth_service: Arc<dyn AuthService>` to `AppState`
- Initialize it in `AppState::new()` alongside user_service

## Error Mapping

| Scenario                  | Error                              |
| ------------------------- | ---------------------------------- |
| Email not found on login  | `AppError::Unauthorized`           |
| Wrong password            | `AppError::Unauthorized`           |
| Duplicate email (register)| `AppError::Conflict`               |
| Invalid input             | `AppError::Validation` (automatic) |

## Thing to Resolve

The users table uses `BIGINT UNSIGNED` for `id`, but `Claims.sub` is a `Uuid`. Options:
- Add a `uuid` column to the users table and use that as the JWT subject
- Change `Claims.sub` to `u64` to match the existing ID
- Store the numeric ID as a string in `sub` and parse it back

Pick one approach before implementing `generate_token`.
