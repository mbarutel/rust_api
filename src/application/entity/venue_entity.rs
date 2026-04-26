use chrono::{DateTime, Utc};

use crate::domain::models::venue::Venue;

#[derive(Debug, sqlx::FromRow)]
pub struct VenueEntity {
    pub id: u64,
    pub name: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<VenueEntity> for Venue {
    fn from(e: VenueEntity) -> Self {
        Venue {
            id: e.id,
            name: e.name,
            address_line1: e.address_line1,
            address_line2: e.address_line2,
            city: e.city,
            state_region: e.state_region,
            postal_code: e.postal_code,
            country: e.country,
            notes: e.notes,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
