use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum BookingStatus {
    Reserved,
    Confirmed,
    Cancelled,
}

impl BookingStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Reserved => "reserved",
            Self::Confirmed => "confirmed",
            Self::Cancelled => "cancelled",
        }
    }
}

impl TryFrom<&str> for BookingStatus {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "reserved" => Ok(Self::Reserved),
            "confirmed" => Ok(Self::Confirmed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActivityBooking {
    pub id: u64,
    pub activity_id: u64,
    pub participant_id: u64,
    pub status: BookingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
