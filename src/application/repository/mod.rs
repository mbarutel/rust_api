pub mod conference_repository;
pub mod user_repository;
pub mod venue_repository;

use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait Repository<T>: Send + Sync {
    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<T>, DomainError>;
    async fn find_by_id(&self, id: u64) -> Result<T, DomainError>;
    async fn create(&self, entity: T) -> Result<T, DomainError>;
    async fn update(&self, entity: T) -> Result<T, DomainError>;
    async fn delete(&self, id: u64) -> Result<(), DomainError>;
    async fn count(&self) -> Result<u64, DomainError>;
}
