use crate::application::entity::exhibitor_entity::ExhibitorEntity;
use crate::application::repository::Repository;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait ExhibitorRepository: Repository<ExhibitorEntity> {
    async fn find_by_participant_id(
        &self,
        participant_id: u64,
    ) -> Result<ExhibitorEntity, DomainError>;
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: ExhibitorEntity,
    ) -> Result<ExhibitorEntity, DomainError>;
}
