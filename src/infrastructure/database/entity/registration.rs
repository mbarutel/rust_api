use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::domain::models::registration::{Registration, RegistrationStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct RegistrationEntity {
    pub id: u64,
    pub conference_id: u64,
    pub status: String,
    pub cost: Decimal,
    pub discount_code: Option<String>,
    pub discount_amount: Decimal,
    pub amount_paid: Decimal,
    pub created_by_id: Option<u64>,
    pub notes_internal: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<RegistrationEntity> for Registration {
    fn from(e: RegistrationEntity) -> Self {
        Registration {
            id: e.id,
            conference_id: e.conference_id,
            status: RegistrationStatus::try_from(e.status.as_str())
                .unwrap_or(RegistrationStatus::Submitted),
            cost: e.cost,
            discount_code: e.discount_code,
            discount_amount: e.discount_amount,
            amount_paid: e.amount_paid,
            created_by_id: e.created_by_id,
            notes_internal: e.notes_internal,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
