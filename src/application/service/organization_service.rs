use crate::{
    application::{
        dto::organization_dto::{CreateOrganizationRequest, UpdateOrganizationRequest},
        error::AppError,
    },
    domain::models::organization::Organization,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait OrganizationService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Organization>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<Organization, AppError>;
    async fn create(&self, dto: CreateOrganizationRequest) -> Result<Organization, AppError>;
    async fn update(
        &self,
        id: u64,
        dto: UpdateOrganizationRequest,
    ) -> Result<Organization, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
}
