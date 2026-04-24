use chrono::{DateTime, Utc};

use crate::domain::models::client::Client;

#[derive(Debug, sqlx::FromRow)]
pub struct ClientEntity {
    pub id: u64,
    pub organization_id: Option<u64>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ClientEntity> for Client {
    fn from(e: ClientEntity) -> Self {
        Client {
            id: e.id,
            organization_id: e.organization_id,
            first_name: e.first_name,
            last_name: e.last_name,
            email: e.email,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
