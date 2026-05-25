use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PriceTier {
    pub id: u64,
    pub conference_id: u64,
    pub price: Decimal,
    pub deadline: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PriceTier {
    // Check if the PriceTier has not expired yet.
    // We don't really care if the user chooses a more
    // expensive price_tier, we only care that the deadline
    // has not passed yet.
    pub fn is_expired(&self) -> bool {
        let today = Utc::now().date_naive();

        // Still valid, has not passed yet
        if today <= self.deadline {
            return true;
        }

        return false;
    }
}
