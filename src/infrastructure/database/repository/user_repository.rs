use sqlx::MySqlPool;

use crate::domain::{error::DomainError, repository::user_repository::UserRepository, user::User};

pub struct DbUserRepository {
    pool: MySqlPool,
}

impl DbUserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for DbUserRepository {
    async fn find_by_id(&self, id: u64) -> Result<User, DomainError> {
        sqlx::query_as!(
            User,
            "SELECT 
                id,
                email,
                first_name,
                last_name,
                password_hash,
                created_at,
                updated_at
            FROM
                users
            WHERE 
                id = ?",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DomainError::NotFound,
            _ => DomainError::Database(e.to_string()),
        })
    }

    async fn find_by_email(&self, email: &str) -> Result<User, DomainError> {
        sqlx::query_as!(
            User,
            "SELECT 
                id,
                email,
                first_name,
                last_name,
                password_hash,
                created_at,
                updated_at
            FROM
                users
            WHERE 
                email = ?",
            email,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DomainError::NotFound,
            _ => DomainError::Database(e.to_string()),
        })
    }

    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<User>, DomainError> {
        sqlx::query_as!(
            User,
            "SELECT 
                id,
                first_name,
                last_name,
                email,
                password_hash,
                created_at,
                updated_at
            FROM
                users
            LIMIT ?
            OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))
    }

    async fn create(&self, user: User) -> Result<User, DomainError> {
        sqlx::query!(
            "INSERT INTO
                users (
                    first_name,
                    last_name,
                    email,
                    password_hash,
                    created_at,
                    updated_at
                ) 
            VALUES (?, ?, ?, ?, ?, ?)",
            user.first_name,
            user.last_name,
            user.email,
            user.password_hash,
            user.created_at,
            user.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, DomainError> {
        sqlx::query!(
            "UPDATE
                users
            SET
                first_name = ?,
                last_name = ?,
                email = ?,
                password_hash = ?,
                updated_at = ?
            WHERE 
                id = ?",
            user.first_name,
            user.last_name,
            user.email,
            user.password_hash,
            user.updated_at,
            user.id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn delete(&self, id: u64) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Database(e.to_string()))?;

        Ok(())
    }

    async fn count(&self) -> Result<u64, DomainError> {
        let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DomainError::Database(e.to_string()))?;

        Ok(count as u64)
    }

    async fn email_exists(&self, email: &str) -> Result<bool, DomainError> {
        let exists =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)", email,)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| DomainError::Database(e.to_string()))?;

        Ok(exists == 1)
    }
}
