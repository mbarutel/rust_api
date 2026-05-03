use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::sponsor_dto::{
    CreateSponsorRequest, SponsorResponse, UpdateSponsorRequest,
};
use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::presentation::middleware::validated_json::ValidateJson;
use crate::state::AppState;

pub fn sponsor_routes() -> Router<AppState> {
    Router::new().route(
        "/api/participants/{id}/sponsor",
        get(find).post(create).put(update).delete(delete),
    )
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<SponsorResponse>, HandlerError> {
    let sponsor = state.services.sponsor.find_by_participant_id(id).await?;
    Ok(Json(SponsorResponse::from(sponsor)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<CreateSponsorRequest>,
) -> Result<Json<SponsorResponse>, HandlerError> {
    let sponsor = state.services.sponsor.create(id, dto).await?;
    Ok(Json(SponsorResponse::from(sponsor)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateSponsorRequest>,
) -> Result<Json<SponsorResponse>, HandlerError> {
    let sponsor = state.services.sponsor.update(id, dto).await?;
    Ok(Json(SponsorResponse::from(sponsor)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.services.sponsor.delete(id).await?;
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
        application::{error::AppError, service::sponsor_service::MockSponsorService},
        domain::{error::DomainError, models::sponsor::Sponsor},
        presentation::handler::{sponsor_handler::sponsor_routes, utils::test_jwt},
        state::{AppState, Services},
    };

    fn fake_sponsor() -> Sponsor {
        Sponsor {
            id: 1,
            participant_id: 1,
            tier: "gold".to_string(),
            company_name: None,
            logo_url: None,
            invoice_contact: None,
            benefits_notes: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn find_ok() {
        let mut svc = MockSponsorService::new();
        svc.expect_find_by_participant_id()
            .once()
            .returning(|_| Ok(fake_sponsor()));

        let app = sponsor_routes().with_state(AppState {
            services: Services {
                sponsor: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/participants/1/sponsor")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut svc = MockSponsorService::new();
        svc.expect_find_by_participant_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = sponsor_routes().with_state(AppState {
            services: Services {
                sponsor: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/participants/99/sponsor")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut svc = MockSponsorService::new();
        svc.expect_delete().once().returning(|_| Ok(()));

        let app = sponsor_routes().with_state(AppState {
            services: Services {
                sponsor: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/participants/1/sponsor")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
