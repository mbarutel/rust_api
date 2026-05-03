use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::domain::models::price_tier::PriceTier;

#[derive(Debug, Deserialize)]
pub struct CreatePriceTierRequest {
    pub price: Decimal,
    pub deadline: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct PriceTierResponse {
    pub id: u64,
    pub price: Decimal,
    pub deadline: NaiveDate,
}

impl From<PriceTier> for PriceTierResponse {
    fn from(pt: PriceTier) -> Self {
        Self {
            id: pt.id,
            price: pt.price,
            deadline: pt.deadline,
        }
    }
}
