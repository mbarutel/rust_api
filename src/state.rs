use crate::{
    application::service::{auth_service::AuthService, user_service::UserService},
    infrastructure::config::Config,
};
use sqlx::mysql::MySqlPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: MySqlPool,
    pub auth_service: Arc<dyn AuthService>,
    pub user_service: Arc<dyn UserService>,
}

// impl AppState {
//     pub async fn new(config: &Config) -> anyhow::Result<Self> {
//         // Initialize database connections
//         let db = MySqlPoolOptions::new()
//             .max_connections(5)
//             .acquire_timeout(Duration::from_secs(5))
//             .connect(&config.database_url)
//             .await
//             .context("Failed to connect to database")?;
//
//         // InitIalize redis cache client
//         // let cache = redis::Client::open(&config.redis_url)
//         // .context("FaileD to connect to Redis");
//         let config = Arc::new(config.clone());
//
//         let user_service = Arc::new(UserServiceImpl::new(db.clone()));
//         let auth_service = Arc::new(AuthServiceImpl::new(
//             db.clone(),
//             config.clone(),
//             user_service.clone(),
//         ));
//
//         Ok(Self {
//             config: config.clone(),
//             db: Some(db.clone()),
//             auth_service,
//             user_service,
//         })
//     }
// }
