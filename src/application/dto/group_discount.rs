use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGroupDiscountRequest {
    pub name: String,
    pub min_quantity: u32,
    pub free_quantity: u32,
    pub valid_until: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGroupDiscountRequest {
    pub name: Option<String>,
    pub min_quantity: Option<u32>,
    pub free_quantity: Option<u32>,
    pub valid_until: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct GroupDiscountResponse {
    pub name: String,
    pub min_quantity: u32,
    pub free_quantity: u32,
    pub is_active: bool,
    pub valid_until: Option<NaiveDateTime>,
}
