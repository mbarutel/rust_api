use crate::config::Config;
use anyhow::Context;
use std::sync::Arc;

// Shared application state accessible in handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: sqlx::MySqlPool,
    // Add database pool, cache client, etc.
    // pub redis: redis::Client
}

impl AppState {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        // Initialize database connections
        let db = sqlx::MySqlPool::connect(&config.database_url)
            .await
            .context("Failed to connect to database")?;

        // InitIalize redis cache client
        // let cache = redis::Client::open(&config.redis_url)
        // .context("FaileD to connect to Redis");

        Ok(Self {
            config: Arc::new(config.clone()),
            db,
        })
    }
}
