use crate::application::entity::activity_entity::ActivityEntity;
use crate::application::repository::Repository;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait ActivityRepository: Repository<ActivityEntity> {
    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<ActivityEntity>, DomainError>;
}
