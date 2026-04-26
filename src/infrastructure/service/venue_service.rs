use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::venue_dto::{CreateVenueRequest, UpdateVenueRequest},
        entity::venue_entity::VenueEntity,
        error::AppError,
        repository::venue_repository::VenueRepository,
        service::venue_service::VenueService,
    },
    domain::models::venue::Venue,
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
        let venues = self
            .venue_repo
            .find_all(offset, per_page)
            .await?
            .into_iter()
            .map(Venue::from)
            .collect();
        Ok((venues, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<Venue, AppError> {
        Ok(Venue::from(self.venue_repo.find_by_id(id).await?))
    }

    async fn create(&self, dto: CreateVenueRequest) -> Result<Venue, AppError> {
        let venue_entity = VenueEntity {
            id: 0,
            name: dto.name,
            address_line1: dto.address_line1,
            address_line2: dto.address_line2,
            city: dto.city,
            state_region: dto.state_region,
            postal_code: dto.postal_code,
            country: dto.country,
            notes: dto.notes,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(Venue::from(self.venue_repo.create(venue_entity).await?))
    }

    async fn update(&self, id: u64, dto: UpdateVenueRequest) -> Result<Venue, AppError> {
        let venue_entity = self.venue_repo.find_by_id(id).await?;
        let venue_entity = VenueEntity {
            name: dto.name.unwrap_or(venue_entity.name),
            address_line1: dto.address_line1.or(venue_entity.address_line1),
            address_line2: dto.address_line2.or(venue_entity.address_line2),
            city: dto.city.or(venue_entity.city),
            state_region: dto.state_region.or(venue_entity.state_region),
            postal_code: dto.postal_code.or(venue_entity.postal_code),
            country: dto.country.or(venue_entity.country),
            notes: dto.notes.or(venue_entity.notes),
            updated_at: Utc::now(),
            ..venue_entity
        };
        Ok(Venue::from(self.venue_repo.update(venue_entity).await?))
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.venue_repo.delete(id).await?)
    }
}
