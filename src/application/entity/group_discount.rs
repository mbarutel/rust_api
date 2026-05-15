use chrono::{DateTime, NaiveDateTime, Utc};

use crate::domain::models::GroupDiscount;

#[derive(Debug, sqlx::FromRow)]
pub struct GroupDiscountEntity {
    pub id: u64,
    pub name: String,
    pub min_quantity: u32,
    pub free_quantity: u32,
    pub active: i8,
    pub valid_until: Option<NaiveDateTime>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GroupDiscountEntity {
    pub fn is_active(&self) -> bool {
        self.active != 0
    }
}

impl From<GroupDiscountEntity> for GroupDiscount {
    fn from(e: GroupDiscountEntity) -> Self {
        let is_active = e.is_active();

        Self {
            id: e.id,
            name: e.name,
            min_quantity: e.min_quantity,
            free_quantity: e.free_quantity,
            is_active: is_active,
            valid_until: e.valid_until,
        }
    }
}
