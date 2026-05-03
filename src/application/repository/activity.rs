use crate::{
    application::{entity::activity::ActivityEntity, repository::Repository},
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait ActivityRepository: Repository<ActivityEntity> {
    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<ActivityEntity>, DomainError>;
}
