use crate::{
    application::{entity::user::UserEntity, repository::Repository},
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait UserRepository: Repository<UserEntity> {
    async fn find_by_email(&self, email: &str) -> Result<UserEntity, DomainError>;
    async fn email_exists(&self, email: &str) -> Result<bool, DomainError>;
}
