use crate::{
    config::Config,
    users::service::{UserService, UserServiceImpl},
};
use anyhow::Context;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::{sync::Arc, time::Duration};

// Shared application state accessible in handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: MySqlPool,
    // Add database pool, cache client, etc.
    // pub redis: redis::Client
    pub user_service: Arc<dyn UserService>,
}

impl AppState {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        // Initialize database connections
        let db = MySqlPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&config.database_url)
            .await
            .context("Failed to connect to database")?;

        // InitIalize redis cache client
        // let cache = redis::Client::open(&config.redis_url)
        // .context("FaileD to connect to Redis");

        Ok(Self {
            config: Arc::new(config.clone()),
            db: db.clone(),
            user_service: Arc::new(UserServiceImpl::new(db)),
        })
    }
}
