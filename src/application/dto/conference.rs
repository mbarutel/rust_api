use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::dto::{GroupDiscountResponse, venue::VenueResponse},
    domain::models::{GroupDiscount, Venue, conference::Conference},
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateConferenceRequest {
    #[validate(length(equal = 4))]
    pub code: String,
    pub name: String,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub venue_id: Option<u64>,
    pub group_discount_id: Option<u64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateConferenceRequest {
    pub name: Option<String>,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub venue_id: Option<u64>,
    pub group_discount_id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ConferenceResponse {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub venue: Option<VenueResponse>,
    pub group_discount: Option<GroupDiscountResponse>,
    pub is_published: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Conference> for ConferenceResponse {
    fn from(conference: Conference) -> Self {
        let start_date = conference.start_date.map(|v| v.to_string());
        let end_date = conference.end_date.map(|v| v.to_string());

        ConferenceResponse {
            id: conference.id,
            code: conference.code,
            name: conference.name,
            poster_url: conference.poster_url,
            description: conference.description,
            start_date,
            end_date,
            venue: None,
            group_discount: None,
            is_published: conference.is_published,
            created_at: conference.created_at.to_string(),
            updated_at: conference.updated_at.to_string(),
        }
    }
}

impl ConferenceResponse {
    pub fn with_venue(mut self, venue: Option<Venue>) -> Self {
        self.venue = venue.map(VenueResponse::from);
        self
    }

    pub fn with_group_discount(mut self, group_discount: Option<GroupDiscount>) -> Self {
        self.group_discount = group_discount.map(GroupDiscountResponse::from);
        self
    }
}
