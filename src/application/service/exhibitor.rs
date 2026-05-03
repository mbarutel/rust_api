use crate::{
    application::{
        dto::exhibitor::{CreateExhibitorRequest, UpdateExhibitorRequest},
        error::AppError,
    },
    domain::models::exhibitor::Exhibitor,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ExhibitorService: Send + Sync {
    async fn find_by_participant_id(&self, participant_id: u64) -> Result<Exhibitor, AppError>;
    async fn create(
        &self,
        participant_id: u64,
        dto: CreateExhibitorRequest,
    ) -> Result<Exhibitor, AppError>;
    async fn update(
        &self,
        participant_id: u64,
        dto: UpdateExhibitorRequest,
    ) -> Result<Exhibitor, AppError>;
    async fn delete(&self, participant_id: u64) -> Result<(), AppError>;
}
