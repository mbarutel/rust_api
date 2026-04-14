use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};

use crate::application::dto::pagination::{ListQueryRequest, PaginatedResponse};
use crate::application::dto::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::presentation::middleware::validated_json::ValidateJson;

use crate::presentation::error::HandlerError;
use crate::presentation::middleware::auth::AuthUser;
use crate::state::AppState;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/api/users", get(list).post(create))
        .route("/api/users/{id}", get(find).put(update).delete(delete))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateUserRequest>,
) -> Result<Json<UserResponse>, HandlerError> {
    let user = state.user_service.create(dto).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateUserRequest>,
) -> Result<Json<UserResponse>, HandlerError> {
    let user = state.user_service.update(id, dto).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<UserResponse>>, HandlerError> {
    let (users, total) = state.user_service.list(query.page, query.per_page).await?;
    let users = users.into_iter().map(UserResponse::from).collect();

    Ok(Json(PaginatedResponse {
        data: users,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn find(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<UserResponse>, HandlerError> {
    let user = state.user_service.find_by_id(id).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.user_service.delete(id).await?;
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
            service::{auth_service::MockAuthService, user_service::MockUserService},
        },
        domain::models::user::User,
        presentation::handler::{
            user_handler::user_routes,
            utils::{test_jwt, test_state},
        },
    };

    fn fake_user() -> User {
        User {
            id: 1,
            first_name: "John".into(),
            last_name: "Doe".into(),
            email: "john@doe.com".into(),
            password_hash: "hash".into(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn list_no_auth() {
        let app =
            user_routes().with_state(test_state(MockUserService::new(), MockAuthService::new()));
        let req = Request::builder()
            .uri("/api/users")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn list_ok() {
        let mut user_service = MockUserService::new();
        user_service
            .expect_list()
            .once()
            .returning(|_, _| Ok((vec![], 0)));

        let app = user_routes().with_state(test_state(user_service, MockAuthService::new()));

        let req = Request::builder()
            .uri("/api/users")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut user_service = MockUserService::new();
        user_service.expect_find_by_id().once().returning(|_| {
            Err(AppError::Domain(
                crate::domain::error::DomainError::NotFound,
            ))
        });

        let app = user_routes().with_state(test_state(user_service, MockAuthService::new()));
        let req = Request::builder()
            .uri("/api/users/99")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn find_ok() {
        let mut user_service = MockUserService::new();
        user_service
            .expect_find_by_id()
            .once()
            .returning(|_| Ok(fake_user()));

        let app = user_routes().with_state(test_state(user_service, MockAuthService::new()));
        let req = Request::builder()
            .uri("/api/users/1")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_conflict() {
        let mut user_service = MockUserService::new();
        user_service.expect_create().once().returning(|_| {
            Err(AppError::Domain(
                crate::domain::error::DomainError::Conflict,
            ))
        });

        let app = user_routes().with_state(test_state(user_service, MockAuthService::new()));
        let req = Request::builder()
             .method("POST")
             .uri("/api/users")
             .header("content-type", "application/json")
             .header("authorization", auth_header())
             .body(Body::from(
                 r#"{"email":"john@doe.com","first_name":"John","last_name":"Doe","password":"password123"}"#,
             ))
             .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn create_invalid_body() {
        let app =
            user_routes().with_state(test_state(MockUserService::new(), MockAuthService::new()));
        let req = Request::builder()
            .method("POST")
            .uri("/api/users")
            .header("content-type", "application/json")
            .header("authorization", auth_header())
            .body(Body::from(
                r#"{"email":"notanemail","first_name":"","last_name":"Doe","password":"short"}"#,
            ))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut users = MockUserService::new();
        users.expect_delete().once().returning(|_| Ok(()));

        let app = user_routes().with_state(test_state(users, MockAuthService::new()));
        let req = Request::builder()
            .method("DELETE")
            .uri("/api/users/1")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
