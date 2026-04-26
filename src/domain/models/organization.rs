use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Organization {
    pub id: u64,
    pub name: String,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
