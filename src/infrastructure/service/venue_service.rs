use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::venue_dto::{CreateVenueRequest, UpdateVenueRequest},
        error::AppError,
        service::venue_service::VenueService,
    },
    domain::{models::venue::Venue, repository::venue_repository::VenueRepository},
};

pub struct VenueServiceImpl {
    venue_repo: Arc<dyn VenueRepository>,
}

impl VenueServiceImpl {
    pub fn new(venue_repo: Arc<dyn VenueRepository>) -> Self {
        Self { venue_repo }
    }
}

#[async_trait::async_trait]
impl VenueService for VenueServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Venue>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.venue_repo.count().await?;
        let venues = self.venue_repo.find_all(offset, per_page).await?;
        Ok((venues, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<Venue, AppError> {
        Ok(self.venue_repo.find_by_id(id).await?)
    }

    async fn create(&self, dto: CreateVenueRequest) -> Result<Venue, AppError> {
        let venue = Venue {
            id: 0,
            name: dto.name,
            address_line1: dto.address_line1,
            address_line2: dto.address_line2.map(Some),
            city: dto.city,
            state_region: dto.state_region,
            postal_code: dto.postal_code,
            country: dto.country,
            notes: dto.notes,
            published: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(self.venue_repo.create(venue).await?)
    }

    async fn update(&self, id: u64, dto: UpdateVenueRequest) -> Result<Venue, AppError> {
        let venue = self.venue_repo.find_by_id(id).await?;
        let venue = Venue {
            id: venue.id,
            name: dto.name.unwrap_or(venue.name),
            address_line1: dto.address_line1.or(venue.address_line1),
            address_line2: dto.address_line2.map(Some).or(venue.address_line2),
            city: dto.city.or(venue.city),
            state_region: dto.state_region.or(venue.state_region),
            postal_code: dto.postal_code.or(venue.postal_code),
            country: dto.country.or(venue.country),
            notes: dto.notes.or(venue.notes),
            published: dto.published.map(|p| p as i8).unwrap_or(venue.published),
            created_at: venue.created_at,
            updated_at: Utc::now(),
        };
        Ok(self.venue_repo.update(venue).await?)
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.venue_repo.delete(id).await?)
    }
}
