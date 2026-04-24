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
    state.conference_service.delete(id).await?;
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
        application::{
            error::AppError,
            service::conference_service::MockConferenceService,
        },
        domain::{error::DomainError, models::conference::Conference},
        presentation::handler::{
            conference_handler::conference_routes,
            utils::test_jwt,
        },
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
}
