use crate::{
    application::{
        dto::sponsor::{CreateSponsorRequest, UpdateSponsorRequest},
        error::AppError,
    },
    domain::models::sponsor::Sponsor,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait SponsorService: Send + Sync {
    async fn find_by_participant_id(&self, participant_id: u64) -> Result<Sponsor, AppError>;
    async fn create(
        &self,
        participant_id: u64,
        dto: CreateSponsorRequest,
    ) -> Result<Sponsor, AppError>;
    async fn update(
        &self,
        participant_id: u64,
        dto: UpdateSponsorRequest,
    ) -> Result<Sponsor, AppError>;
    async fn delete(&self, participant_id: u64) -> Result<(), AppError>;
}
