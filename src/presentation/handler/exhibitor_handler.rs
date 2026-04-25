use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::exhibitor_dto::{
    CreateExhibitorRequest, ExhibitorResponse, UpdateExhibitorRequest,
};
use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::presentation::middleware::validated_json::ValidateJson;
use crate::state::AppState;

pub fn exhibitor_routes() -> Router<AppState> {
    Router::new().route(
        "/api/participants/{id}/exhibitor",
        get(find).post(create).put(update).delete(delete),
    )
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ExhibitorResponse>, HandlerError> {
    let exhibitor = state.exhibitor_service.find_by_participant_id(id).await?;
    Ok(Json(ExhibitorResponse::from(exhibitor)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<CreateExhibitorRequest>,
) -> Result<Json<ExhibitorResponse>, HandlerError> {
    let exhibitor = state.exhibitor_service.create(id, dto).await?;
    Ok(Json(ExhibitorResponse::from(exhibitor)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateExhibitorRequest>,
) -> Result<Json<ExhibitorResponse>, HandlerError> {
    let exhibitor = state.exhibitor_service.update(id, dto).await?;
    Ok(Json(ExhibitorResponse::from(exhibitor)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.exhibitor_service.delete(id).await?;
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
            service::exhibitor_service::MockExhibitorService,
        },
        domain::{error::DomainError, models::exhibitor::Exhibitor},
        presentation::handler::{exhibitor_handler::exhibitor_routes, utils::test_jwt},
        state::AppState,
    };

    fn fake_exhibitor() -> Exhibitor {
        Exhibitor {
            id: 1,
            participant_id: 1,
            company_name: "Acme Corp".to_string(),
            power_required: false,
            internet_required: false,
            notes_internal: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn find_ok() {
        let mut svc = MockExhibitorService::new();
        svc.expect_find_by_participant_id()
            .once()
            .returning(|_| Ok(fake_exhibitor()));

        let app = exhibitor_routes().with_state(AppState {
            exhibitor_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/participants/1/exhibitor")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut svc = MockExhibitorService::new();
        svc.expect_find_by_participant_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = exhibitor_routes().with_state(AppState {
            exhibitor_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/participants/99/exhibitor")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut svc = MockExhibitorService::new();
        svc.expect_delete().once().returning(|_| Ok(()));

        let app = exhibitor_routes().with_state(AppState {
            exhibitor_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/participants/1/exhibitor")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
