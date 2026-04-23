use crate::application::{entity::venue_entity::VenueEntity, repository::Repository};

#[async_trait::async_trait]
pub trait VenueRepository: Repository<VenueEntity> {}
