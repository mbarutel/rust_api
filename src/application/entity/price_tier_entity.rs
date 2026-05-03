use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;

#[derive(Debug, sqlx::FromRow)]
pub struct PriceTierEntity {
    pub id: u64,
    pub conference_id: u64,
    pub price: Decimal,
    pub deadline: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
