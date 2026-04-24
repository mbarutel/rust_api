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
    pub published: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VenueEntity {
    pub fn is_published(&self) -> bool {
        self.published != 0
    }
}

impl From<VenueEntity> for Venue {
    fn from(venue_entity: VenueEntity) -> Self {
        let published = venue_entity.is_published();
        Venue {
            id: venue_entity.id,
            name: venue_entity.name,
            address_line1: venue_entity.address_line1,
            address_line2: venue_entity.address_line2,
            city: venue_entity.city,
            state_region: venue_entity.state_region,
            postal_code: venue_entity.postal_code,
            country: venue_entity.country,
            notes: venue_entity.notes,
            published,
            created_at: venue_entity.created_at,
            updated_at: venue_entity.updated_at,
        }
    }
}
