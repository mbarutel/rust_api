use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::organization::Organization;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrganizationRequest {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(url)]
    pub website: Option<String>,
    pub phone: Option<String>,
    #[validate(email)]
    pub billing_email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrganizationRequest {
    #[validate(length(min = 4))]
    pub name: Option<String>,
    #[validate(url)]
    pub website: Option<String>,
    pub phone: Option<String>,
    #[validate(email)]
    pub billing_email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationResponse {
    pub id: u64,
    pub name: String,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub billing_email: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Organization> for OrganizationResponse {
    fn from(organization: Organization) -> Self {
        OrganizationResponse {
            id: organization.id,
            name: organization.name,
            website: organization.website,
            phone: organization.phone,
            billing_email: organization.billing_email,
            created_at: organization.created_at.to_string(),
            updated_at: organization.updated_at.to_string(),
        }
    }
}
