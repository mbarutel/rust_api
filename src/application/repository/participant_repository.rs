use crate::application::entity::participant_entity::ParticipantEntity;
use crate::application::repository::Repository;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait ParticipantRepository: Repository<ParticipantEntity> {
    async fn find_by_registration(
        &self,
        registration_id: u64,
    ) -> Result<Vec<ParticipantEntity>, DomainError>;
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: ParticipantEntity,
    ) -> Result<ParticipantEntity, DomainError>;
}
