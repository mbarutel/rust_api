use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::error::{AppError, Result};
use crate::state::AppState;

// User Response Model
// Should this be here or in models?
#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String, // TODO: follow that pattern from 21+ tips video by boghdan
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
