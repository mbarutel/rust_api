use chrono::{DateTime, Utc};

use crate::domain::models::sponsor::Sponsor;

#[derive(Debug, sqlx::FromRow)]
pub struct SponsorEntity {
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

impl From<SponsorEntity> for Sponsor {
    fn from(e: SponsorEntity) -> Self {
        Sponsor {
            id: e.id,
            participant_id: e.participant_id,
            tier: e.tier,
            company_name: e.company_name,
            logo_url: e.logo_url,
            invoice_contact: e.invoice_contact,
            benefits_notes: e.benefits_notes,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
