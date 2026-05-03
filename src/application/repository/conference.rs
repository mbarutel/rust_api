use crate::{
    application::{entity::conference::ConferenceEntity, repository::Repository},
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait ConferenceRepository: Repository<ConferenceEntity> {
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: ConferenceEntity,
    ) -> Result<ConferenceEntity, DomainError>;
}
