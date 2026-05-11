use crate::{
    application::{
        dto::speaker::{CreateSpeakerRequest, UpdateSpeakerRequest},
        error::AppError,
    },
    domain::models::speaker::Speaker,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait SpeakerService: Send + Sync {
    async fn find_by_participant_id(&self, participant_id: u64) -> Result<Speaker, AppError>;
    async fn create(
        &self,
        participant_id: u64,
        dto: CreateSpeakerRequest,
    ) -> Result<Speaker, AppError>;
    async fn update(
        &self,
        participant_id: u64,
        dto: UpdateSpeakerRequest,
    ) -> Result<Speaker, AppError>;
    async fn delete(&self, participant_id: u64) -> Result<(), AppError>;
}
