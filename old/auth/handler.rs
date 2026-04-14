use axum::Json;
use axum::extract::State;

use crate::auth::model;
use crate::error::Result;
use crate::middleware::validated_json::ValidateJson;
use crate::state::AppState;

#[tracing::instrument(skip(state, payload), fields(email = %payload.email))]
pub async fn login(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<model::LoginRequest>,
) -> Result<Json<model::TokenResponse>> {
    tracing::info!("User {} logging in", payload.email);

    let token = state.auth_service.login(payload).await?;

    Ok(Json(token))
}

#[tracing::instrument(skip(state, payload), fields(email = %payload.email))]
pub async fn register(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<model::RegisterRequest>,
) -> Result<Json<model::TokenResponse>> {
    tracing::info!("User {} registering", payload.email);

    println!("Registering");
    let token = state.auth_service.register(payload).await?;

    Ok(Json(token))
}
