pub mod auth;
pub mod common;
pub mod config;
pub mod error;
pub mod health;
pub mod middleware;
pub mod state;
pub mod test_helpers;
pub mod users;
// onion
pub mod application;
pub mod domain;

use axum::{Router, http::StatusCode};
use std::time::Duration;
use tower_governor::GovernorLayer;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{config::Config, middleware::rate_limiting::rate_limit_config, state::AppState};

pub fn build_router(state: AppState, config: &Config) -> Router {
    let router = Router::new()
        .merge(health::router())
        .merge(auth::routes::router())
        .merge(users::routes::router());

    let router = if config.rate_limiting {
        router.layer(GovernorLayer::new(rate_limit_config()))
    } else {
        router
    };

    router
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
                let request_id = request
                    .headers()
                    .get("x-request-id")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown");

                tracing::info_span!(
                    "http_request",
                    method = %request.method(),
                    uri = %request.uri(),
                    request_id = %request_id,
                )
            }),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_methods(Any),
        )
        .with_state(state)
}
