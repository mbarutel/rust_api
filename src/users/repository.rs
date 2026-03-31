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
    let result = sqlx::query!(
        "INSERT INTO users (email, name, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        email,
        name,
        password_hash,
        now,
        now,
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(result.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    email: Option<String>,
    name: Option<String>,
) -> Result<UserResponse> {
    let mut set_clauses = Vec::new();
    let mut bindings = Vec::new();

    if let Some(name) = name {
        set_clauses.push("name = ?");
        bindings.push(name);
    }

    if let Some(email) = email {
        set_clauses.push("email = ?");
        bindings.push(email);
    }

    if set_clauses.is_empty() {
        return find_by_id(pool, id).await;
    }

    let sql = format!("UPDATE users SET {} WHERE id = ?", set_clauses.join(", "));

    let mut query = sqlx::query(&sql);
    for binding in &bindings {
        query = query.bind(binding);
    }
    query.bind(id).execute(pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::Internal(e.into()),
    })?;

    find_by_id(pool, id).await
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<bool> {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(result.rows_affected() > 0)
}
