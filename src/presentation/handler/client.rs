use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    application::dto::{
        client::{ClientResponse, CreateClientRequest, UpdateClientRequest},
        pagination::{ListQueryRequest, PaginatedResponse},
    },
    presentation::{
        error::HandlerError,
        middleware::{auth::AuthUser, validated_json::ValidateJson},
    },
    state::AppState,
};

pub fn client_routes() -> Router<AppState> {
    Router::new()
        .route("/api/clients", get(list).post(create))
        .route("/api/clients/{id}", get(find).put(update).delete(delete))
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<ClientResponse>>, HandlerError> {
    let (clients, total) = state
        .services
        .client
        .list(query.page, query.per_page)
        .await?;
    let clients = clients.into_iter().map(ClientResponse::from).collect();

    Ok(Json(PaginatedResponse {
        data: clients,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn find(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ClientResponse>, HandlerError> {
    let client = state.services.client.find_by_id(id).await?;
    Ok(Json(ClientResponse::from(client)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateClientRequest>,
) -> Result<Json<ClientResponse>, HandlerError> {
    let client = state.services.client.create(dto).await?;
    Ok(Json(ClientResponse::from(client)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateClientRequest>,
) -> Result<Json<ClientResponse>, HandlerError> {
    let client = state.services.client.update(id, dto).await?;
    Ok(Json(ClientResponse::from(client)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.services.client.delete(id).await?;
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
        application::{error::AppError, service::client::MockClientService},
        domain::{error::DomainError, models::client::Client},
        presentation::handler::{client::client_routes, utils::test_jwt},
        state::{AppState, Services},
    };

    fn fake_client() -> Client {
        Client {
            id: 1,
            organization_id: None,
            first_name: "Jane".into(),
            last_name: "Doe".into(),
            email: "jane@doe.com".into(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    #[tokio::test]
    async fn list_ok() {
        let mut client_service = MockClientService::new();
        client_service
            .expect_list()
            .once()
            .returning(|_, _| Ok((vec![], 0)));

        let app = client_routes().with_state(AppState {
            services: Services {
                client: Arc::new(client_service),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .uri("/api/clients")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_ok() {
        let mut client_service = MockClientService::new();
        client_service
            .expect_find_by_id()
            .once()
            .returning(|_| Ok(fake_client()));

        let app = client_routes().with_state(AppState {
            services: Services {
                client: Arc::new(client_service),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .uri("/api/clients/1")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn find_not_found() {
        let mut client_service = MockClientService::new();
        client_service
            .expect_find_by_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = client_routes().with_state(AppState {
            services: Services {
                client: Arc::new(client_service),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .uri("/api/clients/99")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn create_conflict() {
        let mut client_service = MockClientService::new();
        client_service
            .expect_create()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::Conflict)));

        let app = client_routes().with_state(AppState {
            services: Services {
                client: Arc::new(client_service),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/clients")
            .header("content-type", "application/json")
            .header("authorization", auth_header())
            .body(Body::from(
                r#"{"first_name":"Jane","last_name":"Doe","email":"jane@doe.com"}"#,
            ))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn delete_ok() {
        let mut client_service = MockClientService::new();
        client_service.expect_delete().once().returning(|_| Ok(()));

        let app = client_routes().with_state(AppState {
            services: Services {
                client: Arc::new(client_service),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .method("DELETE")
            .uri("/api/clients/1")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
