use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::client::Client;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateClientRequest {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub organization_id: Option<u64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateClientRequest {
    #[validate(length(min = 1))]
    pub first_name: Option<String>,
    #[validate(length(min = 1))]
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub organization_id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ClientResponse {
    pub id: u64,
    pub organization_id: Option<u64>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Client> for ClientResponse {
    fn from(c: Client) -> Self {
        ClientResponse {
            id: c.id,
            organization_id: c.organization_id,
            first_name: c.first_name,
            last_name: c.last_name,
            email: c.email,
            created_at: c.created_at.to_string(),
            updated_at: c.updated_at.to_string(),
        }
    }
}
