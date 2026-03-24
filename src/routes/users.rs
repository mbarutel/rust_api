use crate::handlers::users;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/api/users", get(users::list).post(users::create))
        .route(
            "/api/users/{id}",
            get(users::get).put(users::update).delete(users::delete),
        )
}
