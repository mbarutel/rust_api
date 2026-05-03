use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

use crate::domain::models::price_tier::PriceTier;

#[derive(Debug, sqlx::FromRow)]
pub struct PriceTierEntity {
    pub id: u64,
    pub conference_id: u64,
    pub price: Decimal,
    pub deadline: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PriceTierEntity> for PriceTier {
    fn from(e: PriceTierEntity) -> Self {
        Self {
            id: e.id,
            conference_id: e.conference_id,
            price: e.price,
            deadline: e.deadline,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
