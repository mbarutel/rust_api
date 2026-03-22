use crate::config::Config;
use std::sync::Arc;

// Shared application state accessible in handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    // Add database pool, cache client, etc.
    // pub db: sqlx::PgPool
    // pub redis: redis::Client
}

impl AppState {
    pub async fn new(config: &Config) -> Self {
        // Initialize database connections, caches, etc.
        // let db = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        //
        Self {
            config: Arc::new(config.clone()),
        }
    }
}
