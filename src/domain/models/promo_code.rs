use chrono::NaiveDateTime;
use rust_decimal::Decimal;

pub enum DiscountType {
    Percentage,
    Fixed,
}

pub struct PromoCode {
    pub id: u64,
    pub conference_id: u64,
    pub code: String,
    pub discount_type: DiscountType,
    pub amount: Decimal,
    pub max_uses: Option<u32>,
    pub used_count: u32,
    pub valid_until: Option<NaiveDateTime>,
}
