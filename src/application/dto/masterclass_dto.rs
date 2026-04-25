use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::masterclass::{Masterclass, MasterclassInstructor};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMasterclassRequest {
    pub conference_id: u64,
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: Option<u64>,
    pub capacity: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMasterclassRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_at: Option<NaiveDateTime>,
    pub end_at: Option<NaiveDateTime>,
    pub venue_id: Option<u64>,
    pub capacity: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct MasterclassResponse {
    pub id: u64,
    pub conference_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub start_at: String,
    pub end_at: String,
    pub venue_id: Option<u64>,
    pub capacity: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Masterclass> for MasterclassResponse {
    fn from(m: Masterclass) -> Self {
        MasterclassResponse {
            id: m.id,
            conference_id: m.conference_id,
            name: m.name,
            description: m.description,
            start_at: m.start_at.to_string(),
            end_at: m.end_at.to_string(),
            venue_id: m.venue_id,
            capacity: m.capacity,
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddInstructorRequest {
    pub participant_id: u64,
    pub is_lead: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MasterclassInstructorResponse {
    pub masterclass_id: u64,
    pub participant_id: u64,
    pub is_lead: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<MasterclassInstructor> for MasterclassInstructorResponse {
    fn from(i: MasterclassInstructor) -> Self {
        MasterclassInstructorResponse {
            masterclass_id: i.masterclass_id,
            participant_id: i.participant_id,
            is_lead: i.is_lead,
            created_at: i.created_at.to_string(),
            updated_at: i.updated_at.to_string(),
        }
    }
}
