use crate::middleware::{auth::AuthUser, validated_json::ValidateJson};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use super::model::{CreateUserRequest, ListQuery, UpdateUserRequest, UserResponse};
use super::repository;
use crate::error::{AppError, Result};
use crate::state::AppState;

#[tracing::instrument(skip(state, _user))]
pub async fn list(
    State(state): State<AppState>,
    _user: AuthUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<UserResponse>>> {
    tracing::debug!(
        page = query.page,
        per_page = query.per_page,
        "Listing users"
    );
    let users = repository::find_all(&state.db).await?;
    Ok(Json(users))
}

#[tracing::instrument(skip(state, payload, _users), fields(user.email = %payload.email))]
pub async fn create(
    State(state): State<AppState>,
    _users: AuthUser,
    ValidateJson(payload): ValidateJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
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
    let id = repository::insert(
        &state.db,
        &payload.email,
        &payload.name,
        &password_hash,
        now,
    )
    .await?;

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

#[tracing::instrument(skip(state, _users))]
pub async fn get(
    State(state): State<AppState>,
    _users: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<UserResponse>> {
    let user = repository::find_by_id(&state.db, id).await?;
    Ok(Json(user))
}

#[tracing::instrument(skip(state, payload, _user))]
pub async fn update(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(payload): ValidateJson<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    tracing::info!("Updating user");
    let user = repository::update(&state.db, id, payload.email, payload.name).await?;
    Ok(Json(user))
}

#[tracing::instrument(skip(state, _user))]
pub async fn delete(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    tracing::info!("Deleting user");

    if !repository::delete(&state.db, id).await? {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
