use std::sync::Arc;

use chrono::Utc;

use crate::{
    application::{
        dto::organization::{CreateOrganizationRequest, UpdateOrganizationRequest},
        entity::organization::OrganizationEntity,
        error::AppError,
        repository::organization::OrganizationRepository,
        service::organization::OrganizationService,
    },
    domain::models::organization::Organization,
};

pub struct OrganizationServiceImpl {
    organization_repo: Arc<dyn OrganizationRepository>,
}

impl OrganizationServiceImpl {
    pub fn new(organization_repo: Arc<dyn OrganizationRepository>) -> Self {
        Self { organization_repo }
    }
}

#[async_trait::async_trait]
impl OrganizationService for OrganizationServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Organization>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.organization_repo.count().await?;
        let organizations = self
            .organization_repo
            .find_all(offset, per_page)
            .await?
            .into_iter()
            .map(Organization::from)
            .collect();
        Ok((organizations, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<Organization, AppError> {
        Ok(Organization::from(
            self.organization_repo.find_by_id(id).await?,
        ))
    }

    async fn create(&self, dto: CreateOrganizationRequest) -> Result<Organization, AppError> {
        let organization_entity = OrganizationEntity {
            id: 0,
            name: dto.name,
            website: dto.website,
            phone: dto.phone,
            billing_email: dto.billing_email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(Organization::from(
            self.organization_repo.create(organization_entity).await?,
        ))
    }

    async fn update(
        &self,
        id: u64,
        dto: UpdateOrganizationRequest,
    ) -> Result<Organization, AppError> {
        let organization_entity = self.organization_repo.find_by_id(id).await?;
        let organization_entity = OrganizationEntity {
            id: organization_entity.id,
            name: dto.name.unwrap_or(organization_entity.name),
            website: dto.website.or(organization_entity.website),
            phone: dto.phone.or(organization_entity.phone),
            billing_email: dto
                .billing_email
                .unwrap_or(organization_entity.billing_email),
            created_at: organization_entity.created_at,
            updated_at: Utc::now(),
        };
        Ok(Organization::from(
            self.organization_repo.update(organization_entity).await?,
        ))
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.organization_repo.delete(id).await?)
    }
}
