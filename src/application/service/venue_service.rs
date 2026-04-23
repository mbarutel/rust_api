use crate::{
    application::{
        dto::venue_dto::{CreateVenueRequest, UpdateVenueRequest},
        error::AppError,
    },
    domain::models::venue::Venue,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait VenueService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Venue>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<Venue, AppError>;
    async fn create(&self, dto: CreateVenueRequest) -> Result<Venue, AppError>;
    async fn update(&self, id: u64, dto: UpdateVenueRequest) -> Result<Venue, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
}
