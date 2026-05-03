use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::participant::Participant;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateParticipantRequest {
    pub registration_id: u64,
    pub client_id: u64,
    pub role: Option<String>,
    pub dietary_requirements: Option<String>,
    pub accessibility_needs: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateParticipantRequest {
    pub role: Option<String>,
    pub dietary_requirements: Option<String>,
    pub accessibility_needs: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ParticipantResponse {
    pub id: u64,
    pub registration_id: u64,
    pub client_id: u64,
    pub role: String,
    pub dietary_requirements: Option<String>,
    pub accessibility_needs: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Participant> for ParticipantResponse {
    fn from(p: Participant) -> Self {
        ParticipantResponse {
            id: p.id,
            registration_id: p.registration_id,
            client_id: p.client_id,
            role: p.role.as_str().to_string(),
            dietary_requirements: p.dietary_requirements,
            accessibility_needs: p.accessibility_needs,
            created_at: p.created_at.to_string(),
            updated_at: p.updated_at.to_string(),
        }
    }
}
