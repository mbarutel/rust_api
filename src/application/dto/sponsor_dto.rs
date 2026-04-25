use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::sponsor::Sponsor;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSponsorRequest {
    #[validate(length(min = 1))]
    pub tier: String,
    pub company_name: Option<String>,
    pub logo_url: Option<String>,
    pub invoice_contact: Option<String>,
    pub benefits_notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSponsorRequest {
    pub tier: Option<String>,
    pub company_name: Option<String>,
    pub logo_url: Option<String>,
    pub invoice_contact: Option<String>,
    pub benefits_notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SponsorResponse {
    pub id: u64,
    pub participant_id: u64,
    pub tier: String,
    pub company_name: Option<String>,
    pub logo_url: Option<String>,
    pub invoice_contact: Option<String>,
    pub benefits_notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Sponsor> for SponsorResponse {
    fn from(s: Sponsor) -> Self {
        SponsorResponse {
            id: s.id,
            participant_id: s.participant_id,
            tier: s.tier,
            company_name: s.company_name,
            logo_url: s.logo_url,
            invoice_contact: s.invoice_contact,
            benefits_notes: s.benefits_notes,
            created_at: s.created_at.to_string(),
            updated_at: s.updated_at.to_string(),
        }
    }
}
