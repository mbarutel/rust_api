use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::dto::venue_dto::VenueResponse,
    domain::models::{conference::Conference, venue::Venue},
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateConferenceRequest {
    #[validate(length(equal = 4))]
    pub code: String,
    pub name: String,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub venue_id: Option<u64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateConferenceRequest {
    #[validate(length(equal = 4))]
    pub code: Option<String>,
    pub name: Option<String>,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub venue_id: Option<u64>,
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
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<(Conference, Option<Venue>)> for ConferenceResponse {
    fn from((conference, venue): (Conference, Option<Venue>)) -> Self {
        let published = conference.is_published();
        let venue = venue.map(VenueResponse::from);
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
            venue,
            published,
            created_at: conference.created_at.to_string(),
            updated_at: conference.updated_at.to_string(),
        }
    }
}
