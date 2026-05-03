use crate::{
    application::{
        entity::masterclass::{MasterclassEntity, MasterclassInstructorEntity},
        repository::Repository,
    },
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait MasterclassRepository: Repository<MasterclassEntity> {
    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<MasterclassEntity>, DomainError>;
}

#[async_trait::async_trait]
pub trait MasterclassInstructorRepository: Send + Sync {
    async fn add(
        &self,
        masterclass_id: u64,
        participant_id: u64,
        is_lead: bool,
    ) -> Result<(), DomainError>;
    async fn remove(&self, masterclass_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn find_by_masterclass(
        &self,
        masterclass_id: u64,
    ) -> Result<Vec<MasterclassInstructorEntity>, DomainError>;
}
