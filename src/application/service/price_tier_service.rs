use crate::{
    application::{dto::price_tier_dto::CreatePriceTierRequest, error::AppError},
    domain::models::price_tier::PriceTier,
};

// This might be not needed
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait PriceTierService: Send + Sync {
    async fn create_many(
        &self,
        conference_id: u64,
        price_tiers: Vec<CreatePriceTierRequest>,
    ) -> Result<Vec<PriceTier>, AppError>;
    async fn delete_by_conference_id(&self, conference_id: u64) -> Result<(), AppError>;
}
