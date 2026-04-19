use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::venue::Venue;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVenueRequest {
    #[validate(length(min = 1))]
    pub name: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateVenueRequest {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub notes: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct VenueResponse {
    pub id: u64,
    pub name: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub notes: Option<String>,
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Venue> for VenueResponse {
    fn from(venue: Venue) -> Self {
        let published = venue.is_published();
        VenueResponse {
            id: venue.id,
            name: venue.name,
            address_line1: venue.address_line1,
            address_line2: venue.address_line2.flatten(),
            city: venue.city,
            state_region: venue.state_region,
            postal_code: venue.postal_code,
            country: venue.country,
            notes: venue.notes,
            published,
            created_at: venue.created_at.to_string(),
            updated_at: venue.updated_at.to_string(),
        }
    }
}
