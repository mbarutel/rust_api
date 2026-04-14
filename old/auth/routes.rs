use axum::{Router, routing::post};

use crate::{auth::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(handler::login))
        .route("/api/auth/register", post(handler::register))
}
