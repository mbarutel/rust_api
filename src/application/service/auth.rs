use crate::application::{
    dto::auth::{LoginRequest, RegisterRequest, TokenResponse},
    error::AppError,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, dto: LoginRequest) -> Result<TokenResponse, AppError>;
    async fn register(&self, dto: RegisterRequest) -> Result<TokenResponse, AppError>;
}
