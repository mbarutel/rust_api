use async_trait;

use crate::{
    application::{
        dto::user_dto::{CreateUserRequest, UpdateUserRequest},
        error::AppError,
    },
    domain::user::User,
};

#[async_trait::async_trait]
pub trait UserService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<User>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<User, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<User, AppError>;
    async fn create(&self, dto: CreateUserRequest) -> Result<User, AppError>;
    async fn update(&self, id: u64, dto: UpdateUserRequest) -> Result<User, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
}
