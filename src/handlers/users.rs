use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::{AppError, Result};
use crate::state::AppState;

// User Response Model
// Should this be here or in models?
#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Create user request with validation
#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

// Update user request
#[derive(Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
}

// Pagination query parameters
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

// List users with pagination
#[tracing::instrument(skip(state))]
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<UserResponse>>> {
    tracing::debug!(
        page = query.page,
        per_page = query.per_page,
        "Listing users"
    );

    let users = match sqlx::query_as!(
        UserResponse,
        "SELECT id, name, email, created_at, updated_at FROM users",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
    {
        Ok(users) => users,
        Err(err) => return Err(err),
    };

    Ok(Json(users))
}

// Create a new user
#[tracing::instrument(skip(state, payload), fields(user.email = %payload.email))]
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    // Validate request
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    tracing::info!("Creating new user");

    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)",
        payload.email,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    if exists == 1 {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Creating password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::Error::msg(e.to_string())))?
        .to_string();

    let now = chrono::Utc::now();

    let result = sqlx::query!(
        "INSERT INTO users (email, name, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        payload.email,
        payload.name,
        password_hash,
        now,
        now,
    )
    .execute(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    let user = UserResponse {
        id: result.last_insert_id(),
        email: payload.email,
        name: payload.name,
        created_at: now,
        updated_at: now,
    };

    tracing::info!(user.id = %user.id, "User created!");

    Ok((StatusCode::CREATED, Json(user)))
}

// Get user by ID
#[tracing::instrument(skip(state))]
pub async fn get(State(state): State<AppState>, Path(id): Path<u64>) -> Result<Json<UserResponse>> {
    let user = sqlx::query_as!(
        UserResponse,
        "SELECT id, email, name, created_at, updated_at FROM users WHERE id = ?",
        id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::Internal(e.into()),
    })?;

    Ok(Json(user))
}

// Update user
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

    let mut set_clauses = Vec::new();
    let mut bindings = Vec::new();

    if let Some(name) = payload.name {
        set_clauses.push("name = ?");
        bindings.push(name)
    }

    if let Some(email) = payload.email {
        set_clauses.push("email = ?");
        bindings.push(email);
    }

    if set_clauses.is_empty() {
        // Nothing to update, just return the current row
        return sqlx::query_as!(
            UserResponse,
            "SELECT id, email, name, created_at, updated_at FROM users WHERE id = ?",
            id,
        )
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::Internal(e.into()),
        })
        .map(Json);
    }

    let sql = format!("UPDATE users SET {} WHERE id = ?", set_clauses.join(", "));

    let mut query = sqlx::query(&sql);
    for binding in &bindings {
        query = query.bind(binding);
    }
    query
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::Internal(e.into()),
        })?;

    // Fetch the updated row
    sqlx::query_as!(
        UserResponse,
        "SELECT id, email, name, created_at, updated_at FROM users WHERE id = ?",
        id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.into()))
    .map(Json)
}

// Delete user
#[tracing::instrument(skip(state))]
pub async fn delete(State(state): State<AppState>, Path(id): Path<u64>) -> Result<StatusCode> {
    tracing::info!("Deleting user");

    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
