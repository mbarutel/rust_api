use crate::domain::{models::venue::Venue, repository::Repository};

#[async_trait::async_trait]
pub trait VenueRepository: Repository<Venue> {}
