use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct GroupDiscount {
    pub id: u64,
    pub code: String,
    pub name: String, // "3 for 2 deal"
    pub min_quantity: u32,
    pub free_quantity: u32,
    pub is_active: bool,
    pub valid_until: Option<NaiveDateTime>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
