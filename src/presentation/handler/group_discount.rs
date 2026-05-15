use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    application::dto::{
        group_discount::{
            CreateGroupDiscountRequest, GroupDiscountResponse, UpdateGroupDiscountRequest,
        },
        pagination::{ListQueryRequest, PaginatedResponse},
    },
    presentation::{
        error::HandlerError,
        middleware::{auth::AuthUser, validated_json::ValidateJson},
    },
    state::AppState,
};

pub fn group_discount_routes() -> Router<AppState> {
    Router::new()
}

async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<GroupDiscountResponse>>, HandlerError> {
    unimplemented!()
}

async fn find(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<GroupDiscountResponse>, HandlerError> {
    unimplemented!()
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateGroupDiscountRequest>,
) -> Result<Json<GroupDiscountResponse>, HandlerError> {
    unimplemented!()
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<UpdateGroupDiscountRequest>,
) -> Result<Json<GroupDiscountResponse>, HandlerError> {
    unimplemented!()
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    unimplemented!()
}
