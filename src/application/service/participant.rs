use crate::{
    application::{
        dto::participant::{CreateParticipantRequest, UpdateParticipantRequest},
        error::AppError,
    },
    domain::models::participant::Participant,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ParticipantService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Participant>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<Participant, AppError>;
    async fn find_by_registration(
        &self,
        registration_id: u64,
    ) -> Result<Vec<Participant>, AppError>;
    async fn create(&self, dto: CreateParticipantRequest) -> Result<Participant, AppError>;
    async fn update(&self, id: u64, dto: UpdateParticipantRequest)
    -> Result<Participant, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
}
