use crate::{
    application::{entity::sponsor::SponsorEntity, repository::Repository},
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait SponsorRepository: Repository<SponsorEntity> {
    async fn find_by_participant_id(
        &self,
        participant_id: u64,
    ) -> Result<SponsorEntity, DomainError>;
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: SponsorEntity,
    ) -> Result<SponsorEntity, DomainError>;
}
