use chrono::{DateTime, Utc};

use crate::domain::models::activity_booking::{ActivityBooking, BookingStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct ActivityBookingEntity {
    pub id: u64,
    pub activity_id: u64,
    pub participant_id: u64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ActivityBookingEntity> for ActivityBooking {
    fn from(e: ActivityBookingEntity) -> Self {
        ActivityBooking {
            id: e.id,
            activity_id: e.activity_id,
            participant_id: e.participant_id,
            status: BookingStatus::try_from(e.status.as_str())
                .unwrap_or(BookingStatus::Reserved),
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
