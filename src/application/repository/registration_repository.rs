use crate::application::entity::registration_entity::RegistrationEntity;
use crate::application::repository::Repository;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait RegistrationRepository: Repository<RegistrationEntity> {
    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<RegistrationEntity>, DomainError>;
}
