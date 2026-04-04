use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

use crate::state::AppState;

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
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}

// Kubernetes liveness probe
async fn liveness() -> &'static str {
    "Ok"
}

// readiness probe - check dependencies
async fn readiness(State(state): State<AppState>) -> Result<&'static str, &'static str> {
    // Check database connection
    // if Some(state.db).acquire().await.is_err() {
    //     return Err("Database Unavailable");
    // }
    //
    // Ok("Ok")
    match state.db {
        Some(db) => {
            if db.acquire().await.is_err() {
                return Err("Database Unavailable");
            }

            Ok("Ok")
        }
        None => Err("Database Non Initialized"),
    }
}
