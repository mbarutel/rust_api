use crate::{
    application::{entity::registration::RegistrationEntity, repository::Repository},
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait RegistrationRepository: Repository<RegistrationEntity> {
    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<RegistrationEntity>, DomainError>;
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: RegistrationEntity,
    ) -> Result<RegistrationEntity, DomainError>;
}
