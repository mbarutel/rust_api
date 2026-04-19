use crate::domain::error::DomainError;
use crate::domain::models::user::User;
use crate::domain::repository::Repository;

#[async_trait::async_trait]
pub trait UserRepository: Repository<User> {
    async fn find_by_email(&self, email: &str) -> Result<User, DomainError>;
    async fn email_exists(&self, email: &str) -> Result<bool, DomainError>;
}
