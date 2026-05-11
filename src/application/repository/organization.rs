use crate::{
    application::{entity::organization::OrganizationEntity, repository::Repository},
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait OrganizationRepository: Repository<OrganizationEntity> {
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: OrganizationEntity,
    ) -> Result<OrganizationEntity, DomainError>;
}
