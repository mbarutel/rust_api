use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::pagination::{ListQueryRequest, PaginatedResponse};
use crate::application::dto::venue_dto::{CreateVenueRequest, UpdateVenueRequest, VenueResponse};
use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::presentation::middleware::validated_json::ValidateJson;
use crate::state::AppState;

pub fn venue_routes() -> Router<AppState> {
    Router::new()
        .route("/api/venues", get(list).post(create))
        .route("/api/venues/{id}", get(find).put(update).delete(delete))
}

async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<VenueResponse>>, HandlerError> {
    let (venues, total) = state.venue_service.list(query.page, query.per_page).await?;
    let venues = venues.into_iter().map(VenueResponse::from).collect();

    Ok(Json(PaginatedResponse {
        data: venues,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn find(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<VenueResponse>, HandlerError> {
    let venue = state.venue_service.find_by_id(id).await?;
    Ok(Json(VenueResponse::from(venue)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateVenueRequest>,
) -> Result<Json<VenueResponse>, HandlerError> {
    let venue = state.venue_service.create(dto).await?;
    Ok(Json(VenueResponse::from(venue)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateVenueRequest>,
) -> Result<Json<VenueResponse>, HandlerError> {
    let venue = state.venue_service.update(id, dto).await?;
    Ok(Json(VenueResponse::from(venue)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.venue_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{
        application::{
            error::AppError,
            service::{
                auth_service::MockAuthService,
                user_service::MockUserService,
                venue_service::MockVenueService,
            },
        },
        domain::{error::DomainError, models::venue::Venue},
        presentation::handler::{utils::{test_jwt, test_state}, venue_handler::venue_routes},
    };

    fn fake_venue() -> Venue {
        Venue {
            id: 1,
            name: "Convention Center".into(),
            address_line1: None,
            address_line2: None,
            city: None,
            state_region: None,
            postal_code: None,
            country: None,
            notes: None,
            published: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn list_no_auth() {
        let app = venue_routes().with_state(test_state(
            MockUserService::new(),
            MockAuthService::new(),
            MockVenueService::new(),
        ));
        let req = Request::builder()
            .uri("/api/venues")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn list_ok() {
        let mut venue_service = MockVenueService::new();
        venue_service
            .expect_list()
            .once()
            .returning(|_, _| Ok((vec![], 0)));

        let app = venue_routes().with_state(test_state(
            MockUserService::new(),
            MockAuthService::new(),
            venue_service,
        ));
        let req = Request::builder()
            .uri("/api/venues")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut venue_service = MockVenueService::new();
        venue_service
            .expect_find_by_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = venue_routes().with_state(test_state(
            MockUserService::new(),
            MockAuthService::new(),
            venue_service,
        ));
        let req = Request::builder()
            .uri("/api/venues/99")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn find_ok() {
        let mut venue_service = MockVenueService::new();
        venue_service
            .expect_find_by_id()
            .once()
            .returning(|_| Ok(fake_venue()));

        let app = venue_routes().with_state(test_state(
            MockUserService::new(),
            MockAuthService::new(),
            venue_service,
        ));
        let req = Request::builder()
            .uri("/api/venues/1")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut venue_service = MockVenueService::new();
        venue_service.expect_delete().once().returning(|_| Ok(()));

        let app = venue_routes().with_state(test_state(
            MockUserService::new(),
            MockAuthService::new(),
            venue_service,
        ));
        let req = Request::builder()
            .method("DELETE")
            .uri("/api/venues/1")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
