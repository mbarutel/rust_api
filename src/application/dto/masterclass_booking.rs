use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::masterclass_booking::MasterclassBooking;

#[derive(Debug, Deserialize, Validate)]
pub struct BookMasterclassRequest {
    pub participant_id: u64,
}

#[derive(Debug, Serialize)]
pub struct MasterclassBookingResponse {
    pub id: u64,
    pub masterclass_id: u64,
    pub participant_id: u64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<MasterclassBooking> for MasterclassBookingResponse {
    fn from(b: MasterclassBooking) -> Self {
        MasterclassBookingResponse {
            id: b.id,
            masterclass_id: b.masterclass_id,
            participant_id: b.participant_id,
            status: b.status.as_str().to_string(),
            created_at: b.created_at.to_string(),
            updated_at: b.updated_at.to_string(),
        }
    }
}
