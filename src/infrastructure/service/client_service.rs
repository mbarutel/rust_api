use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::client_dto::{CreateClientRequest, UpdateClientRequest},
        entity::client_entity::ClientEntity,
        error::AppError,
        repository::client_repository::ClientRepository,
        service::client_service::ClientService,
    },
    domain::{error::DomainError, models::client::Client},
};

pub struct ClientServiceImpl {
    client_repo: Arc<dyn ClientRepository>,
}

impl ClientServiceImpl {
    pub fn new(client_repo: Arc<dyn ClientRepository>) -> Self {
        Self { client_repo }
    }
}

#[async_trait::async_trait]
impl ClientService for ClientServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Client>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.client_repo.count().await?;
        let clients = self
            .client_repo
            .find_all(offset, per_page)
            .await?
            .into_iter()
            .map(Client::from)
            .collect();
        Ok((clients, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<Client, AppError> {
        Ok(Client::from(self.client_repo.find_by_id(id).await?))
    }

    async fn create(&self, dto: CreateClientRequest) -> Result<Client, AppError> {
        if self.client_repo.email_exists(&dto.email).await? {
            return Err(AppError::Domain(DomainError::Conflict));
        }

        let client = ClientEntity {
            id: 0,
            organization_id: dto.organization_id,
            first_name: dto.first_name,
            last_name: dto.last_name,
            email: dto.email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(Client::from(self.client_repo.create(client).await?))
    }

    async fn update(&self, id: u64, dto: UpdateClientRequest) -> Result<Client, AppError> {
        let client = self.client_repo.find_by_id(id).await?;
        let client = ClientEntity {
            id: client.id,
            organization_id: dto.organization_id.or(client.organization_id),
            first_name: dto.first_name.unwrap_or(client.first_name),
            last_name: dto.last_name.unwrap_or(client.last_name),
            email: dto.email.unwrap_or(client.email),
            created_at: client.created_at,
            updated_at: Utc::now(),
        };

        Ok(Client::from(self.client_repo.update(client).await?))
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.client_repo.delete(id).await?)
    }
}
