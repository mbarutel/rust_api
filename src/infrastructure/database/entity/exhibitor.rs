use chrono::{DateTime, Utc};

use crate::domain::models::exhibitor::Exhibitor;

#[derive(Debug, sqlx::FromRow)]
pub struct ExhibitorEntity {
    pub id: u64,
    pub participant_id: u64,
    pub company_name: String,
    pub power_required: i8,
    pub internet_required: i8,
    pub notes_internal: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ExhibitorEntity> for Exhibitor {
    fn from(e: ExhibitorEntity) -> Self {
        Exhibitor {
            id: e.id,
            participant_id: e.participant_id,
            company_name: e.company_name,
            power_required: e.power_required != 0,
            internet_required: e.internet_required != 0,
            notes_internal: e.notes_internal,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
