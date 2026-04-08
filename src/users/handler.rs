use crate::middleware::{auth::AuthUser, validated_json::ValidateJson};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use super::model::{CreateUserRequest, ListQuery, UpdateUserRequest, UserResponse};
use crate::common::pagination::PaginatedResponse;
use crate::error::{AppError, Result};
use crate::state::AppState;

#[tracing::instrument(skip(state, _user))]
pub async fn list(
    State(state): State<AppState>,
    _user: AuthUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginatedResponse<UserResponse>>> {
    tracing::info!(
        page = query.page,
        per_page = query.per_page,
        "Listing users"
    );

    let (users, total) = state.user_service.list(query.page, query.per_page).await?;

    Ok(Json(PaginatedResponse {
        data: users,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

#[tracing::instrument(skip(state, payload, _users), fields(user.email = %payload.email))]
pub async fn create(
    State(state): State<AppState>,
    _users: AuthUser,
    ValidateJson(payload): ValidateJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    tracing::info!("Creating new user");
    let user = state.user_service.create(payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[tracing::instrument(skip(state, _users))]
pub async fn get(
    State(state): State<AppState>,
    _users: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<UserResponse>> {
    tracing::info!("Getting user");
    let user = state.user_service.get(id).await?;
    Ok(Json(user))
}

#[tracing::instrument(skip(state, payload, _user))]
pub async fn update(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(payload): ValidateJson<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    tracing::info!("Updating user");
    let user = state.user_service.update(id, payload).await?;
    Ok(Json(user))
}

#[tracing::instrument(skip(state, _user))]
pub async fn delete(
    State(state): State<AppState>,
    _user: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    tracing::info!("Deleting user");

    if !state.user_service.delete(id).await? {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use crate::{
        test_helpers::{auth_delete, auth_get, auth_post, auth_put, parse_body},
        users::service::UserService,
    };

    use super::*;
    use axum::Router;
    use std::sync::Arc;
    use tower::ServiceExt;

    type ListResponse = (Vec<UserResponse>, u64);

    pub struct MockUserService {
        create_fn: Box<dyn Fn(CreateUserRequest) -> Result<UserResponse> + Send + Sync>,
        list_fn: Box<dyn Fn(u32, u32) -> Result<ListResponse> + Send + Sync>,
        get_fn: Box<dyn Fn(u64) -> Result<UserResponse> + Send + Sync>,
        update_fn: Box<dyn Fn(u64, UpdateUserRequest) -> Result<UserResponse> + Send + Sync>,
        delete_fn: Box<dyn Fn(u64) -> Result<bool> + Send + Sync>,
    }

    impl Default for MockUserService {
        fn default() -> Self {
            Self {
                create_fn: Box::new(|_| unimplemented!()),
                list_fn: Box::new(|_, _| unimplemented!()),
                get_fn: Box::new(|_| unimplemented!()),
                update_fn: Box::new(|_, _| unimplemented!()),
                delete_fn: Box::new(|_| unimplemented!()),
            }
        }
    }

    #[async_trait::async_trait]
    impl UserService for MockUserService {
        async fn create(&self, payload: CreateUserRequest) -> Result<UserResponse> {
            (self.create_fn)(payload)
        }

        async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<UserResponse>, u64)> {
            (self.list_fn)(page, per_page)
        }

        async fn get(&self, id: u64) -> Result<UserResponse> {
            (self.get_fn)(id)
        }

        async fn update(&self, id: u64, payload: UpdateUserRequest) -> Result<UserResponse> {
            (self.update_fn)(id, payload)
        }

        async fn delete(&self, id: u64) -> Result<bool> {
            (self.delete_fn)(id)
        }
    }

    fn test_app(service: MockUserService) -> Router {
        let config = crate::test_helpers::test_config();
        let state = AppState {
            config: Arc::new(config),
            db: None,
            user_service: Arc::new(service),
        };

        crate::users::routes::router().with_state(state)
    }

    fn fake_user(id: u64) -> UserResponse {
        let now = chrono::Utc::now();
        UserResponse {
            id,
            email: "test@example.com".into(),
            name: "Test User".into(),
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn create_returns_201_on_success() {
        let app = test_app(MockUserService {
            create_fn: Box::new(|_| Ok(fake_user(1))),
            ..Default::default()
        });

        let resp = app
            .oneshot(auth_post(
                "/api/users",
                r#"{"email":"new@example.com","name":"New User","password":"password123"}"#,
            ))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::CREATED);
        let user: UserResponse = parse_body(resp).await;
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn create_returns_409_on_conflict() {
        let app = test_app(MockUserService {
            create_fn: Box::new(|_| Err(AppError::Conflict("User already exists".into()))),
            ..Default::default()
        });

        let resp = app
            .oneshot(auth_post(
                "/api/users",
                r#"{"email":"dup@example.com","name":"Dup User","password":"password123"}"#,
            ))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn get_returns_200_on_success() {
        let app = test_app(MockUserService {
            get_fn: Box::new(|_| Ok(fake_user(1))),
            ..Default::default()
        });

        let resp = app.oneshot(auth_get("/api/users/1")).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let user: UserResponse = parse_body(resp).await;
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn get_returns_404_when_not_found() {
        let app = test_app(MockUserService {
            get_fn: Box::new(|_| Err(AppError::NotFound)),
            ..Default::default()
        });

        let resp = app.oneshot(auth_get("/api/users/999")).await.unwrap();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn list_returns_paginated_users() {
        let app = test_app(MockUserService {
            list_fn: Box::new(|_, _| Ok((vec![fake_user(1), fake_user(2)], 2))),
            ..Default::default()
        });

        let resp = app.oneshot(auth_get("/api/users")).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body: PaginatedResponse<UserResponse> = parse_body(resp).await;
        assert_eq!(body.data.len(), 2);
        assert_eq!(body.total, 2);
    }

    #[tokio::test]
    async fn update_returns_200_on_success() {
        let app = test_app(MockUserService {
            update_fn: Box::new(|_, _| {
                let mut user = fake_user(1);
                user.name = "Updated User".into();
                Ok(user)
            }),
            ..Default::default()
        });

        let resp = app
            .oneshot(auth_put("/api/users/1", r#"{"name":"Updated User"}"#))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let user: UserResponse = parse_body(resp).await;
        assert_eq!(user.name, "Updated User");
    }

    #[tokio::test]
    async fn delete_returns_204_on_success() {
        let app = test_app(MockUserService {
            delete_fn: Box::new(|_| Ok(true)),
            ..Default::default()
        });

        let resp = app.oneshot(auth_delete("/api/users/1")).await.unwrap();

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn delete_returns_404_when_not_found() {
        let app = test_app(MockUserService {
            delete_fn: Box::new(|_| Err(AppError::NotFound)),
            ..Default::default()
        });

        let resp = app.oneshot(auth_delete("/api/users/1")).await.unwrap();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
