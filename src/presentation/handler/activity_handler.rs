use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::activity_booking_dto::{ActivityBookingResponse, BookActivityRequest};
use crate::application::dto::activity_dto::{
    ActivityResponse, CreateActivityRequest, UpdateActivityRequest,
};
use crate::application::dto::pagination::{ListQueryRequest, PaginatedResponse};
use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::presentation::middleware::validated_json::ValidateJson;
use crate::state::AppState;

pub fn activity_routes() -> Router<AppState> {
    Router::new()
        .route("/api/activities", get(list).post(create))
        .route("/api/activities/{id}", get(find).put(update).delete(delete))
        .route("/api/conferences/{id}/activities", get(list_by_conference))
        .route(
            "/api/activities/{id}/bookings",
            get(list_bookings).post(book),
        )
        .route(
            "/api/activities/{id}/bookings/{participant_id}",
            axum::routing::delete(cancel_booking),
        )
        .route(
            "/api/activities/{id}/bookings/{participant_id}/confirm",
            axum::routing::post(confirm_booking),
        )
        .route(
            "/api/participants/{id}/activity-bookings",
            get(list_bookings_by_participant),
        )
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<ActivityResponse>>, HandlerError> {
    let (activities, total) = state
        .services
        .activity
        .list(query.page, query.per_page)
        .await?;
    Ok(Json(PaginatedResponse {
        data: activities.into_iter().map(ActivityResponse::from).collect(),
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn list_by_conference(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Vec<ActivityResponse>>, HandlerError> {
    let activities = state
        .services
        .activity
        .find_by_conference(id)
        .await?
        .into_iter()
        .map(ActivityResponse::from)
        .collect();
    Ok(Json(activities))
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ActivityResponse>, HandlerError> {
    let activity = state.services.activity.find_by_id(id).await?;
    Ok(Json(ActivityResponse::from(activity)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateActivityRequest>,
) -> Result<Json<ActivityResponse>, HandlerError> {
    let activity = state.services.activity.create(dto).await?;
    Ok(Json(ActivityResponse::from(activity)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateActivityRequest>,
) -> Result<Json<ActivityResponse>, HandlerError> {
    let activity = state.services.activity.update(id, dto).await?;
    Ok(Json(ActivityResponse::from(activity)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.services.activity.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_bookings(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Vec<ActivityBookingResponse>>, HandlerError> {
    let bookings = state
        .services
        .activity
        .list_bookings_by_activity(id)
        .await?
        .into_iter()
        .map(ActivityBookingResponse::from)
        .collect();
    Ok(Json(bookings))
}

async fn list_bookings_by_participant(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Vec<ActivityBookingResponse>>, HandlerError> {
    let bookings = state
        .services
        .activity
        .list_bookings_by_participant(id)
        .await?
        .into_iter()
        .map(ActivityBookingResponse::from)
        .collect();
    Ok(Json(bookings))
}

async fn book(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<BookActivityRequest>,
) -> Result<StatusCode, HandlerError> {
    state.services.activity.book(id, dto.participant_id).await?;
    Ok(StatusCode::CREATED)
}

async fn confirm_booking(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path((id, participant_id)): Path<(u64, u64)>,
) -> Result<StatusCode, HandlerError> {
    state
        .services
        .activity
        .confirm_booking(id, participant_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn cancel_booking(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path((id, participant_id)): Path<(u64, u64)>,
) -> Result<StatusCode, HandlerError> {
    state
        .services
        .activity
        .cancel_booking(id, participant_id)
        .await?;
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
        application::{error::AppError, service::activity_service::MockActivityService},
        domain::{
            error::DomainError,
            models::{
                activity::Activity,
                activity_booking::{ActivityBooking, BookingStatus},
            },
        },
        presentation::handler::{activity_handler::activity_routes, utils::test_jwt},
        state::{AppState, Services},
    };

    fn fake_activity() -> Activity {
        Activity {
            id: 1,
            conference_id: 1,
            name: "Workshop".to_string(),
            description: None,
            start_at: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            end_at: chrono::NaiveDateTime::from_timestamp_opt(3600, 0).unwrap(),
            venue_id: None,
            provider_url: None,
            capacity: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn fake_booking() -> ActivityBooking {
        ActivityBooking {
            id: 1,
            activity_id: 1,
            participant_id: 1,
            status: BookingStatus::Reserved,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn list_ok() {
        let mut svc = MockActivityService::new();
        svc.expect_list().once().returning(|_, _| Ok((vec![], 0)));

        let app = activity_routes().with_state(AppState {
            services: Services {
                activity: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/activities")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_ok() {
        let mut svc = MockActivityService::new();
        svc.expect_find_by_id()
            .once()
            .returning(|_| Ok(fake_activity()));

        let app = activity_routes().with_state(AppState {
            services: Services {
                activity: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/activities/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut svc = MockActivityService::new();
        svc.expect_find_by_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = activity_routes().with_state(AppState {
            services: Services {
                activity: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/activities/99")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn list_by_conference_ok() {
        let mut svc = MockActivityService::new();
        svc.expect_find_by_conference()
            .once()
            .returning(|_| Ok(vec![]));

        let app = activity_routes().with_state(AppState {
            services: Services {
                activity: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/conferences/1/activities")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut svc = MockActivityService::new();
        svc.expect_delete().once().returning(|_| Ok(()));

        let app = activity_routes().with_state(AppState {
            services: Services {
                activity: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/activities/1")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn list_bookings_ok() {
        let mut svc = MockActivityService::new();
        svc.expect_list_bookings_by_activity()
            .once()
            .returning(|_| Ok(vec![fake_booking()]));

        let app = activity_routes().with_state(AppState {
            services: Services {
                activity: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/activities/1/bookings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn book_ok() {
        let mut svc = MockActivityService::new();
        svc.expect_book().once().returning(|_, _| Ok(()));

        let app = activity_routes().with_state(AppState {
            services: Services {
                activity: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/activities/1/bookings")
                    .header("authorization", auth_header())
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"participant_id":1}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn cancel_booking_ok() {
        let mut svc = MockActivityService::new();
        svc.expect_cancel_booking().once().returning(|_, _| Ok(()));

        let app = activity_routes().with_state(AppState {
            services: Services {
                activity: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/activities/1/bookings/1")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn confirm_booking_ok() {
        let mut svc = MockActivityService::new();
        svc.expect_confirm_booking().once().returning(|_, _| Ok(()));

        let app = activity_routes().with_state(AppState {
            services: Services {
                activity: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/activities/1/bookings/1/confirm")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
