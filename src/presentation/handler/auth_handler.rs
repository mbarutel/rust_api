use axum::extract::State;
use axum::{Json, Router, routing::post};

use crate::application::dto::auth_dto::{LoginRequest, RegisterRequest, TokenResponse};
use crate::presentation::error::HandlerError;
use crate::presentation::middleware::validated_json::ValidateJson;
use crate::state::AppState;

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

    let token = state.auth_service.login(payload).await?;

    Ok(Json(token))
}

#[tracing::instrument(skip(state, payload), fields(email = %payload.email))]
pub async fn register(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<RegisterRequest>,
) -> Result<Json<TokenResponse>, HandlerError> {
    tracing::info!("User {} registering", payload.email);

    println!("Registering");
    let token = state.auth_service.register(payload).await?;

    Ok(Json(token))
}
