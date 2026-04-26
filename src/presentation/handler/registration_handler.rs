use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::pagination::{ListQueryRequest, PaginatedResponse};
use crate::application::dto::registration_dto::{
    CreateRegistrationRequest, RecordPaymentRequest, RegistrationResponse, TransitionStatusRequest,
    UpdateRegistrationRequest,
};
use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::presentation::middleware::validated_json::ValidateJson;
use crate::state::AppState;

pub fn registration_routes() -> Router<AppState> {
    Router::new()
        .route("/api/registrations", get(list).post(create))
        .route(
            "/api/registrations/{id}",
            get(find).put(update).delete(delete),
        )
        .route(
            "/api/registrations/{id}/status",
            axum::routing::put(transition_status),
        )
        .route(
            "/api/registrations/{id}/payments",
            axum::routing::post(record_payment),
        )
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<RegistrationResponse>>, HandlerError> {
    let (registrations, total) = state
        .registration_service
        .list(query.page, query.per_page)
        .await?;
    let registrations = registrations
        .into_iter()
        .map(RegistrationResponse::from)
        .collect();

    Ok(Json(PaginatedResponse {
        data: registrations,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<RegistrationResponse>, HandlerError> {
    let registration = state.registration_service.find_by_id(id).await?;
    Ok(Json(RegistrationResponse::from(registration)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateRegistrationRequest>,
) -> Result<Json<RegistrationResponse>, HandlerError> {
    let registration = state.registration_service.create(dto).await?;
    Ok(Json(RegistrationResponse::from(registration)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateRegistrationRequest>,
) -> Result<Json<RegistrationResponse>, HandlerError> {
    let registration = state.registration_service.update(id, dto).await?;
    Ok(Json(RegistrationResponse::from(registration)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.registration_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn transition_status(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<TransitionStatusRequest>,
) -> Result<Json<RegistrationResponse>, HandlerError> {
    let registration = state
        .registration_service
        .transition_status(id, dto)
        .await?;
    Ok(Json(RegistrationResponse::from(registration)))
}

async fn record_payment(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    Json(dto): Json<RecordPaymentRequest>,
) -> Result<Json<RegistrationResponse>, HandlerError> {
    let registration = state.registration_service.record_payment(id, dto).await?;
    Ok(Json(RegistrationResponse::from(registration)))
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
        application::{error::AppError, service::registration_service::MockRegistrationService},
        domain::{error::DomainError, models::registration::Registration},
        presentation::handler::{registration_handler::registration_routes, utils::test_jwt},
        state::AppState,
    };

    fn fake_registration() -> Registration {
        use rust_decimal::Decimal;
        Registration {
            id: 1,
            conference_id: 1,
            status: crate::domain::models::registration::RegistrationStatus::Submitted,
            cost: Decimal::ZERO,
            discount_code: None,
            discount_amount: Decimal::ZERO,
            amount_paid: Decimal::ZERO,
            created_by_id: None,
            notes_internal: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn list_ok() {
        let mut svc = MockRegistrationService::new();
        svc.expect_list().once().returning(|_, _| Ok((vec![], 0)));

        let app = registration_routes().with_state(AppState {
            registration_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/registrations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_ok() {
        let mut svc = MockRegistrationService::new();
        svc.expect_find_by_id()
            .once()
            .returning(|_| Ok(fake_registration()));

        let app = registration_routes().with_state(AppState {
            registration_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/registrations/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut svc = MockRegistrationService::new();
        svc.expect_find_by_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = registration_routes().with_state(AppState {
            registration_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/registrations/99")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut svc = MockRegistrationService::new();
        svc.expect_delete().once().returning(|_| Ok(()));

        let app = registration_routes().with_state(AppState {
            registration_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/registrations/1")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn transition_status_invalid() {
        let mut svc = MockRegistrationService::new();
        svc.expect_transition_status().once().returning(|_, _| {
            Err(AppError::Domain(DomainError::InvalidTransition(
                "cannot transition from 'rejected' to 'accepted'".into(),
            )))
        });

        let app = registration_routes().with_state(AppState {
            registration_service: Arc::new(svc),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/api/registrations/1/status")
                    .header("content-type", "application/json")
                    .header("authorization", auth_header())
                    .body(Body::from(r#"{"status":"accepted"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
