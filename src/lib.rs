pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod state;

use crate::{
    infrastructure::config::Config,
    presentation::{
        handler::{
            activity_handler::activity_routes, auth_handler::auth_routes,
            client_handler::client_routes, conference_handler::conference_routes,
            exhibitor_handler::exhibitor_routes, health_handler::health_routes,
            masterclass_handler::masterclass_routes, organization_handler::organization_routes,
            participant_handler::participant_routes, registration_handler::registration_routes,
            speaker_handler::speaker_routes, sponsor_handler::sponsor_routes,
            user_handler::user_routes, venue_handler::venue_routes,
        },
        middleware::rate_limiting::rate_limit_config,
    },
    state::AppState,
};
use axum::{Router, http::StatusCode};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::signal;
use tower_governor::GovernorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub async fn run() -> anyhow::Result<()> {
    init_tracing();

    let config = Arc::new(Config::from_env());
    let state = AppState::init(config.clone()).await?;
    let app = build_router(state, &config);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Server is running on port {}", config.port);

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received ctrl+c, starting graceful shutdown");
        }
        _ = terminate => {
            tracing::info!("Received SIGTERM, starting graceful shutdown");
        }
    }
}

pub fn build_router(state: AppState, config: &Config) -> Router {
    let router = Router::new()
        .merge(health_routes())
        .merge(activity_routes())
        .merge(auth_routes())
        .merge(client_routes())
        .merge(conference_routes())
        .merge(exhibitor_routes())
        .merge(masterclass_routes())
        .merge(organization_routes())
        .merge(participant_routes())
        .merge(registration_routes())
        .merge(speaker_routes())
        .merge(sponsor_routes())
        .merge(user_routes())
        .merge(venue_routes());

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
