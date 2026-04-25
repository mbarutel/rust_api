use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Speaker {
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
