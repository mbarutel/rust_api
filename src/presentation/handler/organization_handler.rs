use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::organization_dto::{
    CreateOrganizationRequest, OrganizationResponse, UpdateOrganizationRequest,
};
use crate::application::dto::pagination::{ListQueryRequest, PaginatedResponse};
use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::presentation::middleware::validated_json::ValidateJson;
use crate::state::AppState;

pub fn organization_routes() -> Router<AppState> {
    Router::new()
        .route("/api/organizations", get(list).post(create))
        .route(
            "/api/organizations/{id}",
            get(find).put(update).delete(delete),
        )
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<OrganizationResponse>>, HandlerError> {
    let (organizations, total) = state
        .services
        .organization
        .list(query.page, query.per_page)
        .await?;
    let organizations = organizations
        .into_iter()
        .map(OrganizationResponse::from)
        .collect();

    Ok(Json(PaginatedResponse {
        data: organizations,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<OrganizationResponse>, HandlerError> {
    let organization = state.services.organization.find_by_id(id).await?;
    Ok(Json(OrganizationResponse::from(organization)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateOrganizationRequest>,
) -> Result<Json<OrganizationResponse>, HandlerError> {
    let organization = state.services.organization.create(dto).await?;
    Ok(Json(OrganizationResponse::from(organization)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateOrganizationRequest>,
) -> Result<Json<OrganizationResponse>, HandlerError> {
    let organization = state.services.organization.update(id, dto).await?;
    Ok(Json(OrganizationResponse::from(organization)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.services.organization.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use std::sync::Arc;

    use crate::{
        application::{error::AppError, service::organization_service::MockOrganizationService},
        domain::{error::DomainError, models::organization::Organization},
        presentation::handler::{organization_handler::organization_routes, utils::test_jwt},
        state::{AppState, Services},
    };

    fn fake_organization() -> Organization {
        Organization {
            id: 1,
            name: "Acme Corp".into(),
            website: Some("https://acme.com".into()),
            phone: Some("+14155550000".into()),
            billing_email: "billing@acme.com".into(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn list_ok() {
        let mut org_service = MockOrganizationService::new();
        org_service
            .expect_list()
            .once()
            .returning(|_, _| Ok((vec![], 0)));

        let app = organization_routes().with_state(AppState {
            services: Services {
                organization: Arc::new(org_service),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .uri("/api/organizations")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_ok() {
        let mut org_service = MockOrganizationService::new();
        org_service
            .expect_find_by_id()
            .once()
            .returning(|_| Ok(fake_organization()));

        let app = organization_routes().with_state(AppState {
            services: Services {
                organization: Arc::new(org_service),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .uri("/api/organizations/1")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut org_service = MockOrganizationService::new();
        org_service
            .expect_find_by_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = organization_routes().with_state(AppState {
            services: Services {
                organization: Arc::new(org_service),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .uri("/api/organizations/99")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut org_service = MockOrganizationService::new();
        org_service.expect_delete().once().returning(|_| Ok(()));

        let app = organization_routes().with_state(AppState {
            services: Services {
                organization: Arc::new(org_service),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .method("DELETE")
            .uri("/api/organizations/1")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
