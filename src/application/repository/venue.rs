use crate::{
    application::{entity::venue::VenueEntity, repository::Repository},
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait VenueRepository: Repository<VenueEntity> {
    async fn find_by_ids(&self, ids: &[u64]) -> Result<Vec<VenueEntity>, DomainError>;
}
