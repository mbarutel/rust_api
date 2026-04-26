use crate::{
    application::{
        dto::conference_dto::{CreateConferenceRequest, UpdateConferenceRequest},
        error::AppError,
    },
    domain::models::conference::Conference,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ConferenceService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Conference>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<Conference, AppError>;
    async fn create(&self, dto: CreateConferenceRequest) -> Result<Conference, AppError>;
    async fn update(&self, id: u64, dto: UpdateConferenceRequest) -> Result<Conference, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
    async fn publish(&self, id: u64, published: bool) -> Result<Conference, AppError>;
}
