use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::pagination::{ListQueryRequest, PaginatedResponse};
use crate::application::dto::participant_dto::{
    CreateParticipantRequest, ParticipantResponse, UpdateParticipantRequest,
};
use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::presentation::middleware::validated_json::ValidateJson;
use crate::state::AppState;

pub fn participant_routes() -> Router<AppState> {
    Router::new()
        .route("/api/participants", get(list).post(create))
        .route(
            "/api/participants/{id}",
            get(find).put(update).delete(delete),
        )
        .route(
            "/api/registrations/{id}/participants",
            get(list_by_registration),
        )
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<ParticipantResponse>>, HandlerError> {
    let (participants, total) = state
        .participant_service
        .list(query.page, query.per_page)
        .await?;
    let participants = participants
        .into_iter()
        .map(ParticipantResponse::from)
        .collect();

    Ok(Json(PaginatedResponse {
        data: participants,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn list_by_registration(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Vec<ParticipantResponse>>, HandlerError> {
    let participants = state
        .participant_service
        .find_by_registration(id)
        .await?
        .into_iter()
        .map(ParticipantResponse::from)
        .collect();

    Ok(Json(participants))
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ParticipantResponse>, HandlerError> {
    let participant = state.participant_service.find_by_id(id).await?;
    Ok(Json(ParticipantResponse::from(participant)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateParticipantRequest>,
) -> Result<Json<ParticipantResponse>, HandlerError> {
    let participant = state.participant_service.create(dto).await?;
    Ok(Json(ParticipantResponse::from(participant)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateParticipantRequest>,
) -> Result<Json<ParticipantResponse>, HandlerError> {
    let participant = state.participant_service.update(id, dto).await?;
    Ok(Json(ParticipantResponse::from(participant)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.participant_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{
        application::{
            error::AppError,
            service::participant_service::MockParticipantService,
        },
        domain::{error::DomainError, models::participant::Participant},
        presentation::handler::{
            participant_handler::participant_routes,
            utils::test_jwt,
        },
        state::AppState,
    };

    fn fake_participant() -> Participant {
        Participant {
            id: 1,
            registration_id: 1,
            client_id: 1,
            role: crate::domain::models::participant::ParticipantRole::Delegate,
            dietary_requirements: None,
            accessibility_needs: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn list_ok() {
        let mut svc = MockParticipantService::new();
        svc.expect_list().once().returning(|_, _| Ok((vec![], 0)));

        let app = participant_routes().with_state(AppState {
            participant_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(Request::builder().uri("/api/participants").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_ok() {
        let mut svc = MockParticipantService::new();
        svc.expect_find_by_id().once().returning(|_| Ok(fake_participant()));

        let app = participant_routes().with_state(AppState {
            participant_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(Request::builder().uri("/api/participants/1").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut svc = MockParticipantService::new();
        svc.expect_find_by_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = participant_routes().with_state(AppState {
            participant_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(Request::builder().uri("/api/participants/99").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn list_by_registration_ok() {
        let mut svc = MockParticipantService::new();
        svc.expect_find_by_registration()
            .once()
            .returning(|_| Ok(vec![]));

        let app = participant_routes().with_state(AppState {
            participant_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/registrations/1/participants")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut svc = MockParticipantService::new();
        svc.expect_delete().once().returning(|_| Ok(()));

        let app = participant_routes().with_state(AppState {
            participant_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/participants/1")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
