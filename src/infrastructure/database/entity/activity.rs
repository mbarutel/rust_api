use chrono::{DateTime, NaiveDateTime, Utc};

use crate::domain::models::activity::Activity;

#[derive(Debug, sqlx::FromRow)]
pub struct ActivityEntity {
    pub id: u64,
    pub conference_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: Option<u64>,
    pub provider_url: Option<String>,
    pub capacity: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ActivityEntity> for Activity {
    fn from(e: ActivityEntity) -> Self {
        Activity {
            id: e.id,
            conference_id: e.conference_id,
            name: e.name,
            description: e.description,
            start_at: e.start_at,
            end_at: e.end_at,
            venue_id: e.venue_id,
            provider_url: e.provider_url,
            capacity: e.capacity,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
