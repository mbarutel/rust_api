use std::sync::Arc;

use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::mysql::MySqlPoolOptions;

use crate::{
    application::{
        dto::auth_dto::Claims,
        service::{
            activity_service::MockActivityService, auth_service::MockAuthService,
            client_service::MockClientService,
            conference_registration_service::MockConferenceRegistrationService,
            conference_service::MockConferenceService,
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
    state::{AppState, Services},
};

impl Default for Services {
    fn default() -> Self {
        Self {
            activity: Arc::new(MockActivityService::new()),
            auth: Arc::new(MockAuthService::new()),
            client: Arc::new(MockClientService::new()),
            conference: Arc::new(MockConferenceService::new()),
            conference_registration: Arc::new(MockConferenceRegistrationService::new()),
            exhibitor: Arc::new(MockExhibitorService::new()),
            masterclass: Arc::new(MockMasterclassService::new()),
            organization: Arc::new(MockOrganizationService::new()),
            participant: Arc::new(MockParticipantService::new()),
            registration: Arc::new(MockRegistrationService::new()),
            speaker: Arc::new(MockSpeakerService::new()),
            sponsor: Arc::new(MockSponsorService::new()),
            user: Arc::new(MockUserService::new()),
            venue: Arc::new(MockVenueService::new()),
        }
    }
}

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
            services: Services::default(),
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
