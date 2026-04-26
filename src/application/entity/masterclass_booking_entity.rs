use chrono::{DateTime, Utc};

use crate::domain::models::masterclass_booking::{BookingStatus, MasterclassBooking};

#[derive(Debug, sqlx::FromRow)]
pub struct MasterclassBookingEntity {
    pub id: u64,
    pub masterclass_id: u64,
    pub participant_id: u64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<MasterclassBookingEntity> for MasterclassBooking {
    fn from(e: MasterclassBookingEntity) -> Self {
        MasterclassBooking {
            id: e.id,
            masterclass_id: e.masterclass_id,
            participant_id: e.participant_id,
            status: BookingStatus::try_from(e.status.as_str())
                .unwrap_or(BookingStatus::Reserved),
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
