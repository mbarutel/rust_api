use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::user_dto::{CreateUserRequest, UpdateUserRequest},
        error::AppError,
        service::user_service::UserService,
    },
    domain::{error::DomainError, repository::user_repository::UserRepository, user::User},
    infrastructure::password::hash_password,
};

pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

#[async_trait::async_trait]
impl UserService for UserServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<User>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.user_repo.count().await?;
        let users = self.user_repo.find_all(offset, per_page).await?;
        Ok((users, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<User, AppError> {
        Ok(self.user_repo.find_by_id(id).await?)
    }

    async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        Ok(self.user_repo.find_by_email(email).await?)
    }

    async fn create(&self, dto: CreateUserRequest) -> Result<User, AppError> {
        if self.user_repo.email_exists(&dto.email).await? {
            return Err(AppError::Domain(DomainError::Conflict));
        }
        let user = User {
            id: 0,
            email: dto.email,
            first_name: dto.first_name,
            last_name: dto.last_name,
            password_hash: hash_password(&dto.password)?,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.user_repo.create(user).await.map_err(AppError::from)
    }

    async fn update(&self, id: u64, dto: UpdateUserRequest) -> Result<User, AppError> {
        let user = self.user_repo.find_by_id(id).await?;
        let password_hash = match dto.password {
            Some(pass) => hash_password(&pass)?,
            None => user.password_hash,
        };
        let user = User {
            id: 0,
            email: dto.email.unwrap_or(user.email),
            first_name: dto.first_name.unwrap_or(user.first_name),
            last_name: dto.last_name.unwrap_or(user.last_name),
            password_hash,
            created_at: user.created_at,
            updated_at: Utc::now(),
        };
        self.user_repo.update(user).await.map_err(AppError::from)
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.user_repo.delete(id).await?)
    }
}
