use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

// // Pagination query parameters
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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn valid_create_request() {
        let req = CreateUserRequest {
            email: "test@example.com".into(),
            name: "Test".into(),
            password: "password123".into(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn invalid_email_rejected() {
        let req = CreateUserRequest {
            email: "not-an-email".into(),
            name: "Test".into(),
            password: "password123".into(),
        };
        assert!(req.validate().is_err())
    }

    #[test]
    fn short_password_rejected() {
        let req = CreateUserRequest {
            email: "test@example.com".into(),
            name: "Test".into(),
            password: "short".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn empty_name_rejected() {
        let req = CreateUserRequest {
            email: "test@example.com".into(),
            name: "".into(),
            password: "password123".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_invalid_email_rejected() {
        let req = UpdateUserRequest {
            email: Some("not-an-email".into()),
            name: None,
        };
        assert!(req.validate().is_err());
    }
}
