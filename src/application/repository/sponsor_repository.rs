use crate::application::entity::sponsor_entity::SponsorEntity;
use crate::application::repository::Repository;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait SponsorRepository: Repository<SponsorEntity> {
    async fn find_by_participant_id(
        &self,
        participant_id: u64,
    ) -> Result<SponsorEntity, DomainError>;
}
