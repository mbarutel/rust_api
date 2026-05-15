use chrono::{DateTime, NaiveDateTime, Utc};

use crate::domain::models::{GroupDiscount, venue::Venue};

#[derive(Debug, Clone)]
pub struct Conference {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub venue: Option<Venue>,
    pub group_discount: Option<GroupDiscount>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
