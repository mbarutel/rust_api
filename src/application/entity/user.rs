use chrono::{DateTime, Utc};

use crate::domain::models::user::User;

#[derive(Debug, sqlx::FromRow)]
pub struct UserEntity {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserEntity> for User {
    fn from(e: UserEntity) -> Self {
        User {
            id: e.id,
            first_name: e.first_name,
            last_name: e.last_name,
            email: e.email,
            password_hash: e.password_hash,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

// NOTE: I still can't find the value of this struct
