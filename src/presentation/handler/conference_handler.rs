use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use rust_decimal::Decimal;

use crate::{
    application::dto::{
        conference_dto::{ConferenceResponse, CreateConferenceRequest, UpdateConferenceRequest},
        pagination::{ListQueryRequest, PaginatedResponse},
        registration_dto::{
            PriceTier, PublicPromoInfo, RegisterDelegateRequest, RegistrationFormData,
            RegistrationResponse,
        },
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
        .route(
            "/api/conferences/{id}/publish",
            axum::routing::post(publish),
        )
        .route(
            "/api/conferences/{id}/unpublish",
            axum::routing::post(unpublish),
        )
        .route(
            "/api/conferences/{id}/registration-form",
            axum::routing::get(registration_form),
        )
        .route(
            "/api/conferences/{id}/register/delegate",
            axum::routing::post(register_delegate),
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
    state.conference_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn publish(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<ConferenceResponse>, HandlerError> {
    let conference = state.conference_service.publish(id, true).await?;
    Ok(Json(ConferenceResponse::from(conference)))
}

async fn unpublish(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<ConferenceResponse>, HandlerError> {
    let conference = state.conference_service.publish(id, false).await?;
    Ok(Json(ConferenceResponse::from(conference)))
}

async fn registration_form(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<RegistrationFormData>, HandlerError> {
    let conference = state.conference_service.find_by_id(id).await?;
    let price_tiers = vec![PriceTier::default(), PriceTier::default()];
    let active_promos = vec![PublicPromoInfo::default(), PublicPromoInfo::default()];
    let form_data = RegistrationFormData {
        conference: ConferenceResponse::from(conference),
        price_tiers: price_tiers,
        active_promos: active_promos,
    };

    Ok(Json(form_data))
}

async fn register_delegate(
    State(state): State<AppState>,
    Json(dto): Json<RegisterDelegateRequest>,
) -> Result<Json<RegistrationResponse>, HandlerError> {
    let registration = state
        .conference_registration_service
        .register_delegates(dto)
        .await?;

    Ok(Json(registration))
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
        application::{error::AppError, service::conference_service::MockConferenceService},
        domain::{error::DomainError, models::conference::Conference},
        presentation::handler::{conference_handler::conference_routes, utils::test_jwt},
        state::AppState,
    };

    fn fake_conference() -> Conference {
        Conference {
            id: 1,
            code: "RUST".into(),
            name: "RustConf 2025".into(),
            poster_url: None,
            description: None,
            start_date: None,
            end_date: None,
            venue: None,
            published: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn list_ok() {
        let mut conference_service = MockConferenceService::new();
        conference_service
            .expect_list()
            .once()
            .returning(|_, _| Ok((vec![], 0)));

        let app = conference_routes().with_state(AppState {
            conference_service: Arc::new(conference_service),
            ..AppState::default()
        });
        let req = Request::builder()
            .uri("/api/conferences")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_ok() {
        let mut conference_service = MockConferenceService::new();
        conference_service
            .expect_find_by_id()
            .once()
            .returning(|_| Ok(fake_conference()));

        let app = conference_routes().with_state(AppState {
            conference_service: Arc::new(conference_service),
            ..AppState::default()
        });
        let req = Request::builder()
            .uri("/api/conferences/1")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut conference_service = MockConferenceService::new();
        conference_service
            .expect_find_by_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = conference_routes().with_state(AppState {
            conference_service: Arc::new(conference_service),
            ..AppState::default()
        });
        let req = Request::builder()
            .uri("/api/conferences/99")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn create_no_auth() {
        let app = conference_routes().with_state(AppState::default());
        let req = Request::builder()
            .method("POST")
            .uri("/api/conferences")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"code":"RUST","name":"RustConf 2025"}"#))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn create_invalid_body() {
        let app = conference_routes().with_state(AppState::default());
        let req = Request::builder()
            .method("POST")
            .uri("/api/conferences")
            .header("content-type", "application/json")
            .header("authorization", auth_header())
            .body(Body::from(r#"{"code":"TOOLONG","name":"RustConf 2025"}"#))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn create_ok() {
        let mut conference_service = MockConferenceService::new();
        conference_service
            .expect_create()
            .once()
            .returning(|_| Ok(fake_conference()));

        let app = conference_routes().with_state(AppState {
            conference_service: Arc::new(conference_service),
            ..AppState::default()
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/conferences")
            .header("content-type", "application/json")
            .header("authorization", auth_header())
            .body(Body::from(r#"{"code":"RUST","name":"RustConf 2025"}"#))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn update_no_auth() {
        let app = conference_routes().with_state(AppState::default());
        let req = Request::builder()
            .method("PUT")
            .uri("/api/conferences/1")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name":"Updated"}"#))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn delete_no_auth() {
        let app = conference_routes().with_state(AppState::default());
        let req = Request::builder()
            .method("DELETE")
            .uri("/api/conferences/1")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut conference_service = MockConferenceService::new();
        conference_service
            .expect_delete()
            .once()
            .returning(|_| Ok(()));

        let app = conference_routes().with_state(AppState {
            conference_service: Arc::new(conference_service),
            ..AppState::default()
        });
        let req = Request::builder()
            .method("DELETE")
            .uri("/api/conferences/1")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn publish_ok() {
        let mut conference_service = MockConferenceService::new();
        conference_service
            .expect_publish()
            .once()
            .withf(|_, published| *published)
            .returning(|_, _| Ok(fake_conference()));

        let app = conference_routes().with_state(AppState {
            conference_service: Arc::new(conference_service),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/conferences/1/publish")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn unpublish_ok() {
        let mut conference_service = MockConferenceService::new();
        conference_service
            .expect_publish()
            .once()
            .withf(|_, published| !published)
            .returning(|_, _| Ok(fake_conference()));

        let app = conference_routes().with_state(AppState {
            conference_service: Arc::new(conference_service),
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/conferences/1/unpublish")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn publish_no_auth() {
        let app = conference_routes().with_state(AppState::default());
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/conferences/1/publish")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }
}
