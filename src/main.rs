use rust_api::{
    build_router,
    infrastructure::{
        config::Config,
        database::{
            pool::create_pool,
            repository::{user_repository::DbUserRepository, venue_repository::DbVenueRepository},
        },
        service::{
            auth_service::AuthServiceImpl, user_service::UserServiceImpl,
            venue_service::VenueServiceImpl,
        },
    },
    state::AppState,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let config = Arc::new(Config::from_env());
    let db_pool = create_pool(&config.database_url).await?;

    let user_repo = Arc::new(DbUserRepository::new(db_pool.clone()));
    let venue_repo = Arc::new(DbVenueRepository::new(db_pool.clone()));

    let venue_service = Arc::new(VenueServiceImpl::new(venue_repo.clone()));
    let user_service = Arc::new(UserServiceImpl::new(user_repo.clone()));
    let auth_service = Arc::new(AuthServiceImpl::new(config.clone(), user_service.clone()));

    // Create application state
    let state = AppState {
        config: config.clone(),
        db: db_pool,
        user_service,
        auth_service,
        venue_service,
    };

    // Build router with all routes and middleware
    let app = build_router(state, &config);

    // Start server with graceful shutdown
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server is running on {}", config.port);
    // axum::serve(listener, app)
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

    tracing::info!("Server shutdown complete");

    Ok(())
}

// Handle graceful shutdown signals
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    // #[cfg(not(unix))]
    // let terminate = std::future::pending::<()>();

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to insall signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Recieved ctrl+c, starting graceful shutdown");
        }
        _ = terminate => {
                tracing::info!("Received SIGTERM, starting graceful shutdown");
        }
    }
}
