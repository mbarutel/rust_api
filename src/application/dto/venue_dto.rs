use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::venue::Venue;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVenueRequest {
    #[validate(length(min = 4))]
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
    #[validate(length(min = 4))]
    pub name: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state_region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub notes: Option<String>,
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
    pub created_at: String,
    pub updated_at: String,
}

impl From<Venue> for VenueResponse {
    fn from(v: Venue) -> Self {
        VenueResponse {
            id: v.id,
            name: v.name,
            address_line1: v.address_line1,
            address_line2: v.address_line2,
            city: v.city,
            state_region: v.state_region,
            postal_code: v.postal_code,
            country: v.country,
            notes: v.notes,
            created_at: v.created_at.to_string(),
            updated_at: v.updated_at.to_string(),
        }
    }
}
