use sqlx::MySqlPool;

use crate::{
    common::password::hash_password,
    error::{AppError, Result},
    users::{
        model::{CreateUserRequest, UpdateUserRequest, UserResponse},
        repository,
    },
};

#[async_trait::async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, payload: CreateUserRequest) -> Result<UserResponse>;
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<UserResponse>, u64)>;
    async fn get(&self, id: u64) -> Result<UserResponse>;
    async fn update(&self, id: u64, payload: UpdateUserRequest) -> Result<UserResponse>;
    async fn delete(&self, id: u64) -> Result<bool>;
}

pub struct UserServiceImpl {
    pool: MySqlPool,
}

impl UserServiceImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, payload: CreateUserRequest) -> Result<UserResponse> {
        if repository::email_exists(&self.pool, &payload.email).await? {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        // Hash password
        let password_hash = hash_password(&payload.password)?;

        // Insert user
        let now = chrono::Utc::now();
        let id = repository::insert(
            &self.pool,
            &payload.email,
            &payload.name,
            &password_hash,
            now,
        )
        .await?;

        Ok(UserResponse {
            id,
            email: payload.email,
            name: payload.name,
            created_at: now,
            updated_at: now,
        })
    }

    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<UserResponse>, u64)> {
        let total = repository::count(&self.pool).await?;
        let users = repository::find_all(&self.pool, page, per_page).await?;
        Ok((users, total))
    }

    async fn get(&self, id: u64) -> Result<UserResponse> {
        repository::find_by_id(&self.pool, id).await
    }

    async fn update(&self, id: u64, payload: UpdateUserRequest) -> Result<UserResponse> {
        repository::update(&self.pool, id, payload.email, payload.name).await
    }

    async fn delete(&self, id: u64) -> Result<bool> {
        repository::delete(&self.pool, id).await
    }
}
