use chrono::{DateTime, Utc};

pub struct GroupDiscount {
    pub id: u64,
    pub conference_id: u64,
    pub name: String, // "3 for 2 deal"
    pub min_quantity: u32,
    pub free_quntity: u32,
    pub is_active: bool,
    pub valid_until: Option<DateTime<Utc>>,
}
