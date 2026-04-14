use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::pagination::{ListQueryRequest, PaginatedResponse};
use crate::application::dto::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::presentation::middleware::validated_json::ValidateJson;

use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::state::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/api/users", get(list).post(create))
        .route("/api/users/{id}", get(find).put(update).delete(delete))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateUserRequest>,
) -> Result<Json<UserResponse>, HandlerError> {
    let user = state.user_service.create(dto).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateUserRequest>,
) -> Result<Json<UserResponse>, HandlerError> {
    let user = state.user_service.update(id, dto).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn list(
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

async fn find(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<UserResponse>, HandlerError> {
    let user = state.user_service.find_by_id(id).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.user_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
