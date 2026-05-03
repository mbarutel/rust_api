use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::speaker_dto::{
    CreateSpeakerRequest, SpeakerResponse, UpdateSpeakerRequest,
};
use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::presentation::middleware::validated_json::ValidateJson;
use crate::state::AppState;

pub fn speaker_routes() -> Router<AppState> {
    Router::new().route(
        "/api/participants/{id}/speaker",
        get(find).post(create).put(update).delete(delete),
    )
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<SpeakerResponse>, HandlerError> {
    let speaker = state.services.speaker.find_by_participant_id(id).await?;
    Ok(Json(SpeakerResponse::from(speaker)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<CreateSpeakerRequest>,
) -> Result<Json<SpeakerResponse>, HandlerError> {
    let speaker = state.services.speaker.create(id, dto).await?;
    Ok(Json(SpeakerResponse::from(speaker)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateSpeakerRequest>,
) -> Result<Json<SpeakerResponse>, HandlerError> {
    let speaker = state.services.speaker.update(id, dto).await?;
    Ok(Json(SpeakerResponse::from(speaker)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.services.speaker.delete(id).await?;
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
        application::{error::AppError, service::speaker_service::MockSpeakerService},
        domain::{error::DomainError, models::speaker::Speaker},
        presentation::handler::{speaker_handler::speaker_routes, utils::test_jwt},
        state::{AppState, Services},
    };

    fn fake_speaker() -> Speaker {
        Speaker {
            id: 1,
            participant_id: 1,
            talk_title: "A great talk".to_string(),
            talk_abstract: None,
            duration_minutes: None,
            av_requirements: None,
            headshot: None,
            bio: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn find_ok() {
        let mut svc = MockSpeakerService::new();
        svc.expect_find_by_participant_id()
            .once()
            .returning(|_| Ok(fake_speaker()));

        let app = speaker_routes().with_state(AppState {
            services: Services {
                speaker: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/participants/1/speaker")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut svc = MockSpeakerService::new();
        svc.expect_find_by_participant_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = speaker_routes().with_state(AppState {
            services: Services {
                speaker: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/participants/99/speaker")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut svc = MockSpeakerService::new();
        svc.expect_delete().once().returning(|_| Ok(()));

        let app = speaker_routes().with_state(AppState {
            services: Services {
                speaker: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/participants/1/speaker")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
