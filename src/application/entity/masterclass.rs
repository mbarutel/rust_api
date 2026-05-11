use chrono::{DateTime, NaiveDateTime, Utc};

use crate::domain::models::masterclass::{Masterclass, MasterclassInstructor};

#[derive(Debug, sqlx::FromRow)]
pub struct MasterclassEntity {
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

impl From<MasterclassEntity> for Masterclass {
    fn from(e: MasterclassEntity) -> Self {
        Masterclass {
            id: e.id,
            conference_id: e.conference_id,
            name: e.name,
            description: e.description,
            start_at: e.start_at,
            end_at: e.end_at,
            venue_id: e.venue_id,
            capacity: e.capacity,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct MasterclassInstructorEntity {
    pub masterclass_id: u64,
    pub participant_id: u64,
    pub is_lead: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<MasterclassInstructorEntity> for MasterclassInstructor {
    fn from(e: MasterclassInstructorEntity) -> Self {
        MasterclassInstructor {
            masterclass_id: e.masterclass_id,
            participant_id: e.participant_id,
            is_lead: e.is_lead != 0,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
