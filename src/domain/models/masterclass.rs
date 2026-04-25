use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct Masterclass {
    pub id: u64,
    pub conference_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: Option<u64>,
    pub capacity: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MasterclassInstructor {
    pub masterclass_id: u64,
    pub participant_id: u64,
    pub is_lead: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
