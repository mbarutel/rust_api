use std::sync::Arc;

use async_trait::async_trait;
use chrono::Duration;
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    application::{
        dto::auth_dto::{Claims, LoginRequest, RegisterRequest, TokenResponse},
        error::AppError,
        service::{auth_service::AuthService, user_service::UserService},
    },
    domain::error::DomainError,
    infrastructure::{config::Config, password::verify_password},
};

pub struct AuthServiceImpl {
    config: Arc<Config>,
    user_service: Arc<dyn UserService>,
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, payload: LoginRequest) -> Result<TokenResponse, AppError> {
        let user = self.user_service.find_by_email(&payload.email).await;
        let user = match user {
            Ok(u) => u,
            Err(e) => match e {
                AppError::Domain(DomainError::NotFound) => return Err(AppError::Unauthorized),
                _ => return Err(e),
            },
        };

        verify_password(&payload.password, &user.password_hash)?;

        let token = generate_token(user.id, &user.email, &self.config)?;

        Ok(TokenResponse { token })
    }

    async fn register(&self, payload: RegisterRequest) -> Result<TokenResponse, AppError> {
        let user = self.user_service.create(payload.into()).await?;

        let token = generate_token(user.id, &user.email, &self.config)?;

        Ok(TokenResponse { token })
    }
}

fn generate_token(user_id: u64, email: &str, config: &Config) -> Result<String, AppError> {
    let now = chrono::Utc::now();

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::hours(24)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}
