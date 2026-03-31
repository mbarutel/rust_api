use axum::{Router, routing::get};

use super::handler;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/users", get(handler::list).post(handler::create))
        .route(
            "/api/users/{id}",
            get(handler::get)
                .put(handler::update)
                .delete(handler::delete),
        )
}
