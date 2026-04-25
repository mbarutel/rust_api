use crate::{
    application::{
        dto::registration_dto::{
            CreateRegistrationRequest, RecordPaymentRequest, TransitionStatusRequest,
            UpdateRegistrationRequest,
        },
        error::AppError,
    },
    domain::models::registration::Registration,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait RegistrationService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Registration>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<Registration, AppError>;
    async fn create(&self, dto: CreateRegistrationRequest) -> Result<Registration, AppError>;
    async fn update(
        &self,
        id: u64,
        dto: UpdateRegistrationRequest,
    ) -> Result<Registration, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
    async fn transition_status(
        &self,
        id: u64,
        dto: TransitionStatusRequest,
    ) -> Result<Registration, AppError>;
    async fn record_payment(
        &self,
        id: u64,
        dto: RecordPaymentRequest,
    ) -> Result<Registration, AppError>;
}
