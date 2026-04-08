use crate::{
    auth::service::{AuthService, AuthServiceImpl},
    config::Config,
    users::service::{UserService, UserServiceImpl},
};
use anyhow::Context;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,

    // NOTE: At the moment, this seems to be redudant because the service will
    // have their own access to the db. There needs to be a smarter way of what struct holds the
    // database connection.
    pub db: Option<MySqlPool>,

    // NOTE: When we need to add cacheing
    // pub redis: redis::Client
    pub auth_service: Arc<dyn AuthService>,
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
        let config = Arc::new(config.clone());

        let user_service = Arc::new(UserServiceImpl::new(db.clone()));
        let auth_service = Arc::new(AuthServiceImpl::new(
            db.clone(),
            config.clone(),
            user_service.clone(),
        ));

        Ok(Self {
            config: config.clone(),
            db: Some(db.clone()),
            auth_service,
            user_service,
        })
    }
}
