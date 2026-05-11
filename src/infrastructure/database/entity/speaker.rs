use chrono::{DateTime, Utc};

use crate::domain::models::speaker::Speaker;

#[derive(Debug, sqlx::FromRow)]
pub struct SpeakerEntity {
    pub id: u64,
    pub participant_id: u64,
    pub talk_title: String,
    pub talk_abstract: Option<String>,
    pub duration_minutes: Option<i32>,
    pub av_requirements: Option<String>,
    pub headshot: Option<String>,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SpeakerEntity> for Speaker {
    fn from(e: SpeakerEntity) -> Self {
        Speaker {
            id: e.id,
            participant_id: e.participant_id,
            talk_title: e.talk_title,
            talk_abstract: e.talk_abstract,
            duration_minutes: e.duration_minutes,
            av_requirements: e.av_requirements,
            headshot: e.headshot,
            bio: e.bio,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
