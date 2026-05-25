use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::dto::{ConferenceResponse, ParticipantInfoRequest, PriceTierResponse};

#[derive(Debug, Serialize)]
pub struct DelegateFormResponse {
    pub conference: ConferenceResponse,
    pub price_tiers: Vec<PriceTierResponse>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DelegateRegistrationRequest {
    pub conference_id: u64,
    pub price_tier_id: u64,
    pub group_discount_code: Option<String>,
    #[validate(length(min = 1))]
    pub delegates: Vec<ParticipantInfoRequest>,
    pub referrer: String,
}
