use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::dto::user_dto::CreateUserRequest;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, max = 100, message = "First name must be 1-100 characters"))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100, message = "Last name must be 1-100 characters"))]
    pub last_name: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64,   // Subject (user ID)
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
    pub email: String,
}

impl From<RegisterRequest> for CreateUserRequest {
    fn from(req: RegisterRequest) -> Self {
        CreateUserRequest {
            email: req.email,
            first_name: req.first_name,
            last_name: req.last_name,
            password: req.password,
        }
    }
}
