use crate::application::entity::speaker_entity::SpeakerEntity;
use crate::application::repository::Repository;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait SpeakerRepository: Repository<SpeakerEntity> {
    async fn find_by_participant_id(
        &self,
        participant_id: u64,
    ) -> Result<SpeakerEntity, DomainError>;
}
