use std::time::Duration;

use anyhow::Context;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn create_pool(database_url: &str) -> anyhow::Result<MySqlPool> {
    MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
        .context("Failed to connect to database")
}
