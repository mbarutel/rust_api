use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
