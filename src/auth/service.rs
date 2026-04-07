use crate::auth::model::{LoginRequest, RegisterRequest, TokenResponse};
use crate::common::password::{hash_password, verify_password};
use crate::config::Config;
use crate::error::{AppError, Result};
use crate::middleware::auth::Claims;
use crate::users::model::UserRow;
use crate::users::repository;
use async_trait::async_trait;
use chrono::Duration;
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::MySqlPool;
use std::sync::Arc;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, payload: LoginRequest) -> Result<TokenResponse>;
    async fn register(&self, payload: RegisterRequest) -> Result<TokenResponse>;
}

pub struct AuthServiceImpl {
    pool: MySqlPool,
    config: Arc<Config>,
}

impl AuthServiceImpl {
    pub fn new(pool: MySqlPool, config: Arc<Config>) -> Self {
        Self { pool, config }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, payload: LoginRequest) -> Result<TokenResponse> {
        let user = repository::find_by_email(&self.pool, &payload.email).await;
        let user: UserRow = match user {
            Ok(u) => u,
            Err(e) => match e {
                AppError::NotFound => return Err(AppError::Unauthorized),
                _ => return Err(e),
            },
        };

        verify_password(&payload.password, &user.password_hash)?;

        let token = generate_token(user.id, &user.email, &self.config)?;

        Ok(TokenResponse { token })
    }

    async fn register(&self, payload: RegisterRequest) -> Result<TokenResponse> {
        if repository::email_exists(&self.pool, &payload.email).await? {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        let password_hash = hash_password(&payload.password)?;
        let now = chrono::Utc::now();
        let id = repository::insert(
            &self.pool,
            &payload.email,
            &payload.name,
            &password_hash,
            now,
        )
        .await?;

        let token = generate_token(id, &payload.email, &self.config)?;

        Ok(TokenResponse { token })
    }
}

fn generate_token(user_id: u64, email: &str, config: &Config) -> Result<String> {
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
    .map_err(|e| AppError::Internal(e.into()))
}
