use std::sync::Arc;

use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::mysql::MySqlPoolOptions;

use crate::{
    application::{
        dto::auth_dto::Claims,
        service::{auth_service::MockAuthService, user_service::MockUserService},
    },
    infrastructure::config::Config,
    state::AppState,
};

// Builds AppState with injected mocks
pub fn test_state(user_svc: MockUserService, auth_svc: MockAuthService) -> AppState {
    AppState {
        config: Arc::new(Config {
            port: 3000,
            rate_limiting: false,
            environment: "test".to_string(),
            database_url: "mysql://fake".to_string(),
            jwt_secret: "test_secret".to_string(),
        }),
        // connect_lazY: pool erxists but never connect unless .acquire() is called
        db: MySqlPoolOptions::new()
            .connect_lazy("mysql://fake")
            .unwrap(),
        auth_service: Arc::new(auth_svc),
        user_service: Arc::new(user_svc),
    }
}

// Generate a valid JWT signed with "test_secret"
pub fn test_jwt(user_id: u64) -> String {
    let claims = Claims {
        sub: user_id,
        email: "test@email.com".into(),
        iat: 0,
        exp: usize::MAX,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"test_secret"),
    )
    .unwrap()
}
