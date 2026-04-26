use crate::application::entity::activity_booking_entity::ActivityBookingEntity;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait ActivityBookingRepository: Send + Sync {
    async fn book(&self, activity_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn confirm(&self, activity_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn cancel(&self, activity_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn find_by_activity(
        &self,
        activity_id: u64,
    ) -> Result<Vec<ActivityBookingEntity>, DomainError>;
    async fn find_by_participant(
        &self,
        participant_id: u64,
    ) -> Result<Vec<ActivityBookingEntity>, DomainError>;
}
