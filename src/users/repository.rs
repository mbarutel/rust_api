use sqlx::MySqlPool;

use super::model::UserResponse;
use crate::error::{AppError, Result};

pub async fn find_all(pool: &MySqlPool) -> Result<Vec<UserResponse>> {
    sqlx::query_as!(
        UserResponse,
        "SELECT id, name, email, created_at, updated_at FROM users",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<UserResponse> {
    sqlx::query_as!(
        UserResponse,
        "SELECT id, email, name, created_at, updated_at FROM users WHERE id = ?",
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::Internal(e.into()),
    })
}

pub async fn email_exists(pool: &MySqlPool, email: &str) -> Result<bool> {
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)", email,)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(exists == 1)
}

pub async fn insert(
    pool: &MySqlPool,
    email: &str,
    name: &str,
    password_hash: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Result<u64> {
    todo!()
}
