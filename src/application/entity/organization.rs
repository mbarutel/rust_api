use chrono::{DateTime, Utc};

use crate::domain::models::organization::Organization;

#[derive(Debug, sqlx::FromRow)]
pub struct OrganizationEntity {
    pub id: u64,
    pub name: String,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub billing_email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<OrganizationEntity> for Organization {
    fn from(organization_entity: OrganizationEntity) -> Self {
        Organization {
            id: organization_entity.id,
            name: organization_entity.name,
            website: organization_entity.website,
            phone: organization_entity.phone,
            billing_email: organization_entity.billing_email,
            created_at: organization_entity.created_at,
            updated_at: organization_entity.updated_at,
        }
    }
}
