use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::speaker::Speaker;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSpeakerRequest {
    #[validate(length(min = 1))]
    pub talk_title: String,
    pub talk_abstract: Option<String>,
    pub duration_minutes: Option<i32>,
    pub av_requirements: Option<String>,
    pub headshot: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSpeakerRequest {
    pub talk_title: Option<String>,
    pub talk_abstract: Option<String>,
    pub duration_minutes: Option<i32>,
    pub av_requirements: Option<String>,
    pub headshot: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SpeakerResponse {
    pub id: u64,
    pub participant_id: u64,
    pub talk_title: String,
    pub talk_abstract: Option<String>,
    pub duration_minutes: Option<i32>,
    pub av_requirements: Option<String>,
    pub headshot: Option<String>,
    pub bio: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Speaker> for SpeakerResponse {
    fn from(s: Speaker) -> Self {
        SpeakerResponse {
            id: s.id,
            participant_id: s.participant_id,
            talk_title: s.talk_title,
            talk_abstract: s.talk_abstract,
            duration_minutes: s.duration_minutes,
            av_requirements: s.av_requirements,
            headshot: s.headshot,
            bio: s.bio,
            created_at: s.created_at.to_string(),
            updated_at: s.updated_at.to_string(),
        }
    }
}
