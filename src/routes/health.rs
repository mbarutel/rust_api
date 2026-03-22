use axum::{Json, Router, routing::get};
use serde::Serialize;

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

// Basic health check
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healty",
        version: env!("CARGO_PKG_VERSION"),
    })
}

// Kubernetes liveness probe
async fn liveness() -> &'static str {
    "Ok"
}

// Kubernetes readiness probe - check dependencies
// async fn readiness(
//     State(state): State<crate::state::AppState>,
// ) -> Result<&'static str, &'static str> {
async fn readiness() -> Result<&'static str, &'static str> {
    // Check database connection
    // if state.db.acquire().await.is_err() {
    //      return Err("Database Unavailable");
    // }

    Ok("Ok")
}
