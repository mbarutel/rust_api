use crate::middleware::{auth::AuthUser, validated_json::ValidateJson};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use super::model::{CreateUserRequest, ListQuery, UpdateUserRequest, UserResponse};
use crate::common::pagination::PaginatedResponse;
use crate::error::{AppError, Result};
use crate::state::AppState;

#[tracing::instrument(skip(state, _user))]
pub async fn list(
    State(state): State<AppState>,
    _user: AuthUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginatedResponse<UserResponse>>> {
    tracing::debug!(
        page = query.page,
        per_page = query.per_page,
        "Listing users"
    );

    let (users, total) = state.user_service.list(query.page, query.per_page).await?;

    Ok(Json(PaginatedResponse {
        data: users,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

#[tracing::instrument(skip(state, payload, _users), fields(user.email = %payload.email))]
pub async fn create(
    State(state): State<AppState>,
    _users: AuthUser,
    ValidateJson(payload): ValidateJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    tracing::info!("Creating new user");
    let user = state.user_service.create(payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[tracing::instrument(skip(state, _users))]
pub async fn get(
    State(state): State<AppState>,
    _users: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<UserResponse>> {
    tracing::info!("Getting user");
    let user = state.user_service.get(id).await?;
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
    let user = state.user_service.update(id, payload).await?;
    Ok(Json(user))
}

#[tracing::instrument(skip(state, _user))]
pub async fn delete(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    tracing::info!("Deleting user");

    if !state.user_service.delete(id).await? {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
