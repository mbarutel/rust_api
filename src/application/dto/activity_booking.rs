use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::activity_booking::ActivityBooking;

#[derive(Debug, Deserialize, Validate)]
pub struct BookActivityRequest {
    pub participant_id: u64,
}

#[derive(Debug, Serialize)]
pub struct ActivityBookingResponse {
    pub id: u64,
    pub activity_id: u64,
    pub participant_id: u64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ActivityBooking> for ActivityBookingResponse {
    fn from(b: ActivityBooking) -> Self {
        ActivityBookingResponse {
            id: b.id,
            activity_id: b.activity_id,
            participant_id: b.participant_id,
            status: b.status.as_str().to_string(),
            created_at: b.created_at.to_string(),
            updated_at: b.updated_at.to_string(),
        }
    }
}
