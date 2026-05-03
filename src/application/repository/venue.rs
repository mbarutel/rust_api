use crate::application::{entity::venue::VenueEntity, repository::Repository};

#[async_trait::async_trait]
pub trait VenueRepository: Repository<VenueEntity> {}
