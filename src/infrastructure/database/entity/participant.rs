use chrono::{DateTime, Utc};

use crate::domain::models::participant::{Participant, ParticipantRole};

#[derive(Debug, sqlx::FromRow)]
pub struct ParticipantEntity {
    pub id: u64,
    pub registration_id: u64,
    pub client_id: u64,
    pub participant_role: String,
    pub dietary_requirements: Option<String>,
    pub accessibility_needs: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ParticipantEntity> for Participant {
    fn from(e: ParticipantEntity) -> Self {
        Participant {
            id: e.id,
            registration_id: e.registration_id,
            client_id: e.client_id,
            role: ParticipantRole::try_from(e.participant_role.as_str())
                .unwrap_or(ParticipantRole::Delegate),
            dietary_requirements: e.dietary_requirements,
            accessibility_needs: e.accessibility_needs,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
