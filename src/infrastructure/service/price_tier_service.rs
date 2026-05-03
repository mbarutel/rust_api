use std::sync::Arc;

use chrono::Utc;
use sqlx::MySqlPool;

use crate::{
    application::{
        dto::price_tier_dto::CreatePriceTierRequest, entity::price_tier_entity::PriceTierEntity,
        error::AppError, service::price_tier_service::PriceTierService,
    },
    domain::models::price_tier::PriceTier,
    infrastructure::database::repository::price_tier_repository::PriceTierRepository,
};

pub struct PriceTierServiceImpl {
    pool: MySqlPool,
    pub price_tier_repo: Arc<dyn PriceTierRepository>,
}

impl PriceTierServiceImpl {
    pub fn new(pool: MySqlPool, price_tier_repo: Arc<dyn PriceTierRepository>) -> Self {
        Self {
            pool,
            price_tier_repo,
        }
    }
}

// I think this is not needed
#[async_trait::async_trait]
impl PriceTierService for PriceTierServiceImpl {
    async fn create_many(
        &self,
        conference_id: u64,
        price_tiers: Vec<CreatePriceTierRequest>,
    ) -> Result<Vec<PriceTier>, AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            AppError::Domain(crate::domain::error::DomainError::Database(e.to_string()))
        })?;
        let now = Utc::now();
        let price_tiers: Vec<PriceTierEntity> = price_tiers
            .into_iter()
            .map(|pt| PriceTierEntity {
                id: 0,
                conference_id,
                price: pt.price,
                deadline: pt.deadline,
                created_at: now,
                updated_at: now,
            })
            .collect();

        let price_tiers = self
            .price_tier_repo
            .create_many_in_tx(&mut tx, price_tiers)
            .await?;

        Ok(price_tiers.into_iter().map(PriceTier::from).collect())
    }

    async fn delete_by_conference_id(&self, conference_id: u64) -> Result<(), AppError> {
        self.price_tier_repo
            .delete_by_conference_id(conference_id)
            .await
            .map_err(|e| AppError::Domain(e))
    }
}
