use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Sponsor {
    pub id: u64,
    pub participant_id: u64,
    pub tier: String,
    pub company_name: Option<String>,
    pub logo_url: Option<String>,
    pub invoice_contact: Option<String>,
    pub benefits_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
