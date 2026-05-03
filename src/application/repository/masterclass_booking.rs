use crate::{
    application::entity::masterclass_booking::MasterclassBookingEntity, domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait MasterclassBookingRepository: Send + Sync {
    async fn book(&self, masterclass_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn confirm(&self, masterclass_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn cancel(&self, masterclass_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn find_by_masterclass(
        &self,
        masterclass_id: u64,
    ) -> Result<Vec<MasterclassBookingEntity>, DomainError>;
    async fn find_by_participant(
        &self,
        participant_id: u64,
    ) -> Result<Vec<MasterclassBookingEntity>, DomainError>;
}
