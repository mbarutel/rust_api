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

// Create user request with validation
#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

// Update user request
#[derive(Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
}

// Pagination query parameters
#[derive(Deserialize, Debug)]
pub struct ListQuery {
    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}
fn default_per_page() -> u32 {
    10
}

// List users with pagination
#[tracing::instrument(skip(state))]
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<UserResponse>>> {
    tracing::debug!(
        page = query.page,
        per_page = query.per_page,
        "Listing users"
    );

    // In production query from database
    let users = vec![UserResponse {
        id: Uuid::new_v4(),
        email: "user@example.com".to_string(),
        name: "Example User".to_string(),
        created_at: chrono::Utc::now(),
    }];

    Ok(Json(users))
}
