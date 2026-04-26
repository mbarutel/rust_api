use crate::application::entity::client_entity::ClientEntity;
use crate::application::repository::Repository;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait ClientRepository: Repository<ClientEntity> {
    async fn find_by_email(&self, email: &str) -> Result<ClientEntity, DomainError>;
    async fn email_exists(&self, email: &str) -> Result<bool, DomainError>;
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: ClientEntity,
    ) -> Result<ClientEntity, DomainError>;
}
