use crate::application::dto::auth_dto::{LoginRequest, RegisterRequest, TokenResponse};
use crate::application::error::AppError;

#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, dto: LoginRequest) -> Result<TokenResponse, AppError>;
    async fn register(&self, dto: RegisterRequest) -> Result<TokenResponse, AppError>;
}
