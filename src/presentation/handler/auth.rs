use axum::{Json, Router, extract::State, routing::post};

use crate::{
    application::dto::auth::{LoginRequest, RegisterRequest, TokenResponse},
    presentation::{error::HandlerError, middleware::validated_json::ValidateJson},
    state::AppState,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
}

#[tracing::instrument(skip(state, payload), fields(email = %payload.email))]
pub async fn login(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<LoginRequest>,
) -> Result<Json<TokenResponse>, HandlerError> {
    tracing::info!("User {} logging in", payload.email);

    let token = state.services.auth.login(payload).await?;

    Ok(Json(token))
}

#[tracing::instrument(skip(state, payload), fields(email = %payload.email))]
pub async fn register(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<RegisterRequest>,
) -> Result<Json<TokenResponse>, HandlerError> {
    tracing::info!("User {} registering", payload.email);

    println!("Registering");
    let token = state.services.auth.register(payload).await?;

    Ok(Json(token))
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
        application::{dto::auth::TokenResponse, error::AppError, service::auth::MockAuthService},
        presentation::handler::auth::auth_routes,
        state::{AppState, Services},
    };

    #[tokio::test]
    async fn login_ok() {
        let mut auth = MockAuthService::new();
        auth.expect_login().once().returning(|_| {
            Ok(TokenResponse {
                token: "tok".into(),
            })
        });

        let app = auth_routes().with_state(AppState {
            services: Services {
                auth: Arc::new(auth),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"email":"a@b.com","password":"secret"}"#))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn login_unauthorized() {
        let mut auth = MockAuthService::new();
        auth.expect_login()
            .once()
            .returning(|_| Err(AppError::Unauthorized));

        let app = auth_routes().with_state(AppState {
            services: Services {
                auth: Arc::new(auth),
                ..Services::default()
            },
            ..AppState::default()
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"email":"a@b.com","password":"secret"}"#))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn login_invalid_email() {
        let app = auth_routes().with_state(AppState::default());
        let req = Request::builder()
            .method("POST")
            .uri("/api/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"email":"notanemail","password":"secret"}"#))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    // TODO: Registration tests
}
