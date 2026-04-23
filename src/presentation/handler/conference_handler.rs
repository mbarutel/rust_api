use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    application::dto::{
        conference_dto::{ConferenceResponse, CreateConferenceRequest, UpdateConferenceRequest},
        pagination::{ListQueryRequest, PaginatedResponse},
    },
    presentation::{
        error::HandlerError,
        middleware::{auth::AuthUser, validated_json::ValidateJson},
    },
    state::AppState,
};

pub fn conference_routes() -> Router<AppState> {
    Router::new()
        .route("/api/conferences", get(list).post(create))
        .route(
            "/api/conferences/{id}",
            get(find).put(update).delete(delete),
        )
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<ConferenceResponse>>, HandlerError> {
    let (conferences, total) = state
        .conference_service
        .list(query.page, query.per_page)
        .await?;
    let conferences = conferences
        .into_iter()
        .map(ConferenceResponse::from)
        .collect();
    Ok(Json(PaginatedResponse {
        data: conferences,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ConferenceResponse>, HandlerError> {
    let conference = state.conference_service.find_by_id(id).await?;
    Ok(Json(ConferenceResponse::from(conference)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateConferenceRequest>,
) -> Result<Json<ConferenceResponse>, HandlerError> {
    let conference = state.conference_service.create(dto).await?;
    Ok(Json(ConferenceResponse::from(conference)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateConferenceRequest>,
) -> Result<Json<ConferenceResponse>, HandlerError> {
    let conference = state.conference_service.update(id, dto).await?;
    Ok(Json(ConferenceResponse::from(conference)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.venue_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
