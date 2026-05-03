use crate::{
    application::{entity::speaker::SpeakerEntity, repository::Repository},
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait SpeakerRepository: Repository<SpeakerEntity> {
    async fn find_by_participant_id(
        &self,
        participant_id: u64,
    ) -> Result<SpeakerEntity, DomainError>;
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: SpeakerEntity,
    ) -> Result<SpeakerEntity, DomainError>;
}
