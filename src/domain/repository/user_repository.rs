use crate::domain::error::DomainError;
use crate::domain::models::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: u64) -> Result<User, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<User, DomainError>;
    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<User>, DomainError>;
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn update(&self, user: User) -> Result<User, DomainError>;
    async fn delete(&self, id: u64) -> Result<(), DomainError>;
    async fn count(&self) -> Result<u64, DomainError>;
    async fn email_exists(&self, email: &str) -> Result<bool, DomainError>;
}
