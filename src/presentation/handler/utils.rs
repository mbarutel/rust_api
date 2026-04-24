use std::sync::Arc;

use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::mysql::MySqlPoolOptions;

use crate::{
    application::{
        dto::auth_dto::Claims,
        service::{
            auth_service::MockAuthService, client_service::MockClientService,
            conference_service::MockConferenceService,
            organization_service::MockOrganizationService, user_service::MockUserService,
            venue_service::MockVenueService,
        },
    },
    infrastructure::config::Config,
    state::AppState,
};

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Arc::new(Config {
                port: 3000,
                rate_limiting: false,
                environment: "test".to_string(),
                database_url: "mysql://fake".to_string(),
                jwt_secret: "test_secret".to_string(),
            }),
            db: MySqlPoolOptions::new()
                .connect_lazy("mysql://fake")
                .unwrap(),
            auth_service: Arc::new(MockAuthService::new()),
            client_service: Arc::new(MockClientService::new()),
            user_service: Arc::new(MockUserService::new()),
            venue_service: Arc::new(MockVenueService::new()),
            conference_service: Arc::new(MockConferenceService::new()),
            organization_service: Arc::new(MockOrganizationService::new()),
        }
    }
}

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
