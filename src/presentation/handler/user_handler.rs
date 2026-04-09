use axum::Json;
use axum::extract::{Path, Query, State};

use crate::application::dto::pagination::{ListQueryRequest, PaginatedResponse};
use crate::application::dto::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::middleware::auth::AuthUser;
use crate::middleware::validated_json::ValidateJson;
use crate::presentation::error::HandlerError;
use crate::state::AppState;

pub async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateUserRequest>,
) -> Result<Json<UserResponse>, HandlerError> {
    let user = state.user_service.create(dto).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateUserRequest>,
) -> Result<Json<UserResponse>, HandlerError> {
    let user = state.user_service.update(id, dto).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<UserResponse>>, HandlerError> {
    let (users, total) = state.user_service.list(query.page, query.per_page).await?;
    let users = users.into_iter().map(UserResponse::from).collect();

    Ok(Json(PaginatedResponse {
        data: users,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}
