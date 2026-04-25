use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::exhibitor::Exhibitor;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateExhibitorRequest {
    #[validate(length(min = 1))]
    pub company_name: String,
    pub power_required: Option<bool>,
    pub internet_required: Option<bool>,
    pub notes_internal: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateExhibitorRequest {
    pub company_name: Option<String>,
    pub power_required: Option<bool>,
    pub internet_required: Option<bool>,
    pub notes_internal: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExhibitorResponse {
    pub id: u64,
    pub participant_id: u64,
    pub company_name: String,
    pub power_required: bool,
    pub internet_required: bool,
    pub notes_internal: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Exhibitor> for ExhibitorResponse {
    fn from(e: Exhibitor) -> Self {
        ExhibitorResponse {
            id: e.id,
            participant_id: e.participant_id,
            company_name: e.company_name,
            power_required: e.power_required,
            internet_required: e.internet_required,
            notes_internal: e.notes_internal,
            created_at: e.created_at.to_string(),
            updated_at: e.updated_at.to_string(),
        }
    }
}
