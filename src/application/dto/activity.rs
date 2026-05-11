use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::activity::Activity;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateActivityRequest {
    pub conference_id: u64,
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: Option<u64>,
    pub provider_url: Option<String>,
    pub capacity: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateActivityRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_at: Option<NaiveDateTime>,
    pub end_at: Option<NaiveDateTime>,
    pub venue_id: Option<u64>,
    pub provider_url: Option<String>,
    pub capacity: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ActivityResponse {
    pub id: u64,
    pub conference_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub start_at: String,
    pub end_at: String,
    pub venue_id: Option<u64>,
    pub provider_url: Option<String>,
    pub capacity: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Activity> for ActivityResponse {
    fn from(a: Activity) -> Self {
        ActivityResponse {
            id: a.id,
            conference_id: a.conference_id,
            name: a.name,
            description: a.description,
            start_at: a.start_at.to_string(),
            end_at: a.end_at.to_string(),
            venue_id: a.venue_id,
            provider_url: a.provider_url,
            capacity: a.capacity,
            created_at: a.created_at.to_string(),
            updated_at: a.updated_at.to_string(),
        }
    }
}
