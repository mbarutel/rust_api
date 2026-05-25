use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::dto::ConferenceResponse;

#[derive(Debug, Serialize)]
pub struct SpeakerFormResponse {
    pub conference: ConferenceResponse,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Paper {
    #[validate(length(min = 50, max = 100))]
    title: String,
    #[validate(length(min = 100, max = 500))]
    description: String,
}

// #[derive(Debug, Deserialize, Validate)]
// pub struct Speaker {
//     participant_info: ParticipantInfo,
//     #[validate(length(min = 50))]
//     biography: String,
// }

#[derive(Debug, Deserialize, Validate)]
pub struct SpeakerRegistrationRequest {
    pub conference_id: u64,
    pub paper: Paper,
}
