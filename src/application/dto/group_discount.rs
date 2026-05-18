use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::GroupDiscount;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGroupDiscountRequest {
    pub name: String,
    pub code: String,
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
    pub code: String,
    pub min_quantity: u32,
    pub free_quantity: u32,
    pub is_active: bool,
    pub valid_until: Option<NaiveDateTime>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<GroupDiscount> for GroupDiscountResponse {
    fn from(gp: GroupDiscount) -> Self {
        Self {
            name: gp.name,
            code: gp.code,
            min_quantity: gp.min_quantity,
            free_quantity: gp.free_quantity,
            is_active: gp.is_active,
            valid_until: gp.valid_until,
            created_at: gp.created_at,
            updated_at: gp.updated_at,
        }
    }
}
