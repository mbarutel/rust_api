use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    application::dto::{
        masterclass::{
            AddInstructorRequest, CreateMasterclassRequest, MasterclassInstructorResponse,
            MasterclassResponse, UpdateMasterclassRequest,
        },
        masterclass_booking::{BookMasterclassRequest, MasterclassBookingResponse},
        pagination::{ListQueryRequest, PaginatedResponse},
    },
    presentation::{
        error::HandlerError,
        middleware::{auth::AuthUser, validated_json::ValidateJson},
    },
    state::AppState,
};

pub fn masterclass_routes() -> Router<AppState> {
    Router::new()
        .route("/api/masterclasses", get(list).post(create))
        .route(
            "/api/masterclasses/{id}",
            get(find).put(update).delete(delete),
        )
        .route(
            "/api/conferences/{id}/masterclasses",
            get(list_by_conference),
        )
        .route(
            "/api/masterclasses/{id}/instructors",
            get(list_instructors).post(add_instructor),
        )
        .route(
            "/api/masterclasses/{id}/instructors/{participant_id}",
            axum::routing::delete(remove_instructor),
        )
        .route(
            "/api/masterclasses/{id}/bookings",
            get(list_bookings).post(book),
        )
        .route(
            "/api/masterclasses/{id}/bookings/{participant_id}",
            axum::routing::delete(cancel_booking),
        )
        .route(
            "/api/masterclasses/{id}/bookings/{participant_id}/confirm",
            axum::routing::post(confirm_booking),
        )
        .route(
            "/api/participants/{id}/masterclass-bookings",
            get(list_bookings_by_participant),
        )
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<MasterclassResponse>>, HandlerError> {
    let (masterclasses, total) = state
        .services
        .masterclass
        .list(query.page, query.per_page)
        .await?;
    Ok(Json(PaginatedResponse {
        data: masterclasses
            .into_iter()
            .map(MasterclassResponse::from)
            .collect(),
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn list_by_conference(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Vec<MasterclassResponse>>, HandlerError> {
    let masterclasses = state
        .services
        .masterclass
        .find_by_conference(id)
        .await?
        .into_iter()
        .map(MasterclassResponse::from)
        .collect();
    Ok(Json(masterclasses))
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<MasterclassResponse>, HandlerError> {
    let masterclass = state.services.masterclass.find_by_id(id).await?;
    Ok(Json(MasterclassResponse::from(masterclass)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateMasterclassRequest>,
) -> Result<Json<MasterclassResponse>, HandlerError> {
    let masterclass = state.services.masterclass.create(dto).await?;
    Ok(Json(MasterclassResponse::from(masterclass)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateMasterclassRequest>,
) -> Result<Json<MasterclassResponse>, HandlerError> {
    let masterclass = state.services.masterclass.update(id, dto).await?;
    Ok(Json(MasterclassResponse::from(masterclass)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.services.masterclass.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_instructors(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Vec<MasterclassInstructorResponse>>, HandlerError> {
    let instructors = state
        .services
        .masterclass
        .list_instructors(id)
        .await?
        .into_iter()
        .map(MasterclassInstructorResponse::from)
        .collect();
    Ok(Json(instructors))
}

async fn add_instructor(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<AddInstructorRequest>,
) -> Result<StatusCode, HandlerError> {
    state
        .services
        .masterclass
        .add_instructor(id, dto.participant_id, dto.is_lead.unwrap_or(false))
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn remove_instructor(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path((id, participant_id)): Path<(u64, u64)>,
) -> Result<StatusCode, HandlerError> {
    state
        .services
        .masterclass
        .remove_instructor(id, participant_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_bookings(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Vec<MasterclassBookingResponse>>, HandlerError> {
    let bookings = state
        .services
        .masterclass
        .list_bookings_by_masterclass(id)
        .await?
        .into_iter()
        .map(MasterclassBookingResponse::from)
        .collect();
    Ok(Json(bookings))
}

async fn list_bookings_by_participant(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Vec<MasterclassBookingResponse>>, HandlerError> {
    let bookings = state
        .services
        .masterclass
        .list_bookings_by_participant(id)
        .await?
        .into_iter()
        .map(MasterclassBookingResponse::from)
        .collect();
    Ok(Json(bookings))
}

async fn book(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<BookMasterclassRequest>,
) -> Result<StatusCode, HandlerError> {
    state
        .services
        .masterclass
        .book(id, dto.participant_id)
        .await?;
    Ok(StatusCode::CREATED)
}

async fn confirm_booking(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path((id, participant_id)): Path<(u64, u64)>,
) -> Result<StatusCode, HandlerError> {
    state
        .services
        .masterclass
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
        .masterclass
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
        application::{error::AppError, service::masterclass::MockMasterclassService},
        domain::{
            error::DomainError,
            models::{
                masterclass::Masterclass,
                masterclass_booking::{BookingStatus, MasterclassBooking},
            },
        },
        presentation::handler::{masterclass::masterclass_routes, utils::test_jwt},
        state::{AppState, Services},
    };

    fn fake_masterclass() -> Masterclass {
        Masterclass {
            id: 1,
            conference_id: 1,
            name: "Rust deep dive".to_string(),
            description: None,
            start_at: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            end_at: chrono::NaiveDateTime::from_timestamp_opt(7200, 0).unwrap(),
            venue_id: None,
            capacity: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn fake_booking() -> MasterclassBooking {
        MasterclassBooking {
            id: 1,
            masterclass_id: 1,
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
        let mut svc = MockMasterclassService::new();
        svc.expect_list().once().returning(|_, _| Ok((vec![], 0)));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/masterclasses")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_ok() {
        let mut svc = MockMasterclassService::new();
        svc.expect_find_by_id()
            .once()
            .returning(|_| Ok(fake_masterclass()));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/masterclasses/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut svc = MockMasterclassService::new();
        svc.expect_find_by_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/masterclasses/99")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn list_instructors_ok() {
        let mut svc = MockMasterclassService::new();
        svc.expect_list_instructors()
            .once()
            .returning(|_| Ok(vec![]));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/masterclasses/1/instructors")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn remove_instructor_ok() {
        let mut svc = MockMasterclassService::new();
        svc.expect_remove_instructor()
            .once()
            .returning(|_, _| Ok(()));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/masterclasses/1/instructors/5")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut svc = MockMasterclassService::new();
        svc.expect_delete().once().returning(|_| Ok(()));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/masterclasses/1")
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
        let mut svc = MockMasterclassService::new();
        svc.expect_list_bookings_by_masterclass()
            .once()
            .returning(|_| Ok(vec![fake_booking()]));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/masterclasses/1/bookings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn book_ok() {
        let mut svc = MockMasterclassService::new();
        svc.expect_book().once().returning(|_, _| Ok(()));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/masterclasses/1/bookings")
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
        let mut svc = MockMasterclassService::new();
        svc.expect_cancel_booking().once().returning(|_, _| Ok(()));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/api/masterclasses/1/bookings/1")
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
        let mut svc = MockMasterclassService::new();
        svc.expect_confirm_booking().once().returning(|_, _| Ok(()));

        let app = masterclass_routes().with_state(AppState {
            services: Services {
                masterclass: Arc::new(svc),
                ..Services::default()
            },
            ..AppState::default()
        });
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/masterclasses/1/bookings/1/confirm")
                    .header("authorization", auth_header())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
