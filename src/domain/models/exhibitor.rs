use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Exhibitor {
    pub id: u64,
    pub participant_id: u64,
    pub company_name: String,
    pub power_required: bool,
    pub internet_required: bool,
    pub notes_internal: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
