use crate::{
    application::{
        dto::client_dto::{CreateClientRequest, UpdateClientRequest},
        error::AppError,
    },
    domain::models::client::Client,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ClientService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Client>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<Client, AppError>;
    async fn create(&self, dto: CreateClientRequest) -> Result<Client, AppError>;
    async fn update(&self, id: u64, dto: UpdateClientRequest) -> Result<Client, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
}
