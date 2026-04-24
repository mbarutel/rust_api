use crate::application::{entity::organization_entity::OrganizationEntity, repository::Repository};

#[async_trait::async_trait]
pub trait OrganizationRepository: Repository<OrganizationEntity> {}
