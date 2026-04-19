use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct Conference {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub venue_id: Option<u64>,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
