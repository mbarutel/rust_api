use std::sync::Arc;

use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::mysql::MySqlPoolOptions;

use crate::{
    application::{
        dto::auth_dto::Claims,
        service::{
            activity_service::MockActivityService, auth_service::MockAuthService,
            client_service::MockClientService, conference_service::MockConferenceService,
            exhibitor_service::MockExhibitorService,
            masterclass_service::MockMasterclassService,
            organization_service::MockOrganizationService,
            participant_service::MockParticipantService,
            registration_service::MockRegistrationService,
            speaker_service::MockSpeakerService, sponsor_service::MockSponsorService,
            user_service::MockUserService, venue_service::MockVenueService,
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
            activity_service: Arc::new(MockActivityService::new()),
            auth_service: Arc::new(MockAuthService::new()),
            client_service: Arc::new(MockClientService::new()),
            conference_service: Arc::new(MockConferenceService::new()),
            exhibitor_service: Arc::new(MockExhibitorService::new()),
            masterclass_service: Arc::new(MockMasterclassService::new()),
            organization_service: Arc::new(MockOrganizationService::new()),
            participant_service: Arc::new(MockParticipantService::new()),
            registration_service: Arc::new(MockRegistrationService::new()),
            speaker_service: Arc::new(MockSpeakerService::new()),
            sponsor_service: Arc::new(MockSponsorService::new()),
            user_service: Arc::new(MockUserService::new()),
            venue_service: Arc::new(MockVenueService::new()),
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
