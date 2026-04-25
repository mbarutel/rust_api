use crate::{
    application::service::{
        auth_service::AuthService, client_service::ClientService,
        conference_service::ConferenceService, exhibitor_service::ExhibitorService,
        organization_service::OrganizationService, participant_service::ParticipantService,
        registration_service::RegistrationService, speaker_service::SpeakerService,
        sponsor_service::SponsorService, user_service::UserService, venue_service::VenueService,
    },
    infrastructure::{
        config::Config,
        database::{
            pool::create_pool,
            repository::{
                client_repository::DbClientRepository,
                conference_repository::DbConferenceRepository,
                exhibitor_repository::DbExhibitorRepository,
                organization_repository::DbOrganizationRepository,
                participant_repository::DbParticipantRepository,
                registration_repository::DbRegistrationRepository,
                speaker_repository::DbSpeakerRepository,
                sponsor_repository::DbSponsorRepository,
                user_repository::DbUserRepository,
                venue_repository::DbVenueRepository,
            },
        },
        service::{
            auth_service::AuthServiceImpl, client_service::ClientServiceImpl,
            conference_service::ConferenceServiceImpl,
            exhibitor_service::ExhibitorServiceImpl,
            organization_service::OrganizationServiceImpl,
            participant_service::ParticipantServiceImpl,
            registration_service::RegistrationServiceImpl,
            speaker_service::SpeakerServiceImpl,
            sponsor_service::SponsorServiceImpl,
            user_service::UserServiceImpl,
            venue_service::VenueServiceImpl,
        },
    },
};
use sqlx::mysql::MySqlPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: MySqlPool,
    pub auth_service: Arc<dyn AuthService>,
    pub client_service: Arc<dyn ClientService>,
    pub exhibitor_service: Arc<dyn ExhibitorService>,
    pub participant_service: Arc<dyn ParticipantService>,
    pub registration_service: Arc<dyn RegistrationService>,
    pub speaker_service: Arc<dyn SpeakerService>,
    pub sponsor_service: Arc<dyn SponsorService>,
    pub user_service: Arc<dyn UserService>,
    pub venue_service: Arc<dyn VenueService>,
    pub conference_service: Arc<dyn ConferenceService>,
    pub organization_service: Arc<dyn OrganizationService>,
}

impl AppState {
    pub async fn init(config: Arc<Config>) -> anyhow::Result<Self> {
        let db = create_pool(&config.database_url).await?;

        let client_repo = Arc::new(DbClientRepository::new(db.clone()));
        let exhibitor_repo = Arc::new(DbExhibitorRepository::new(db.clone()));
        let participant_repo = Arc::new(DbParticipantRepository::new(db.clone()));
        let registration_repo = Arc::new(DbRegistrationRepository::new(db.clone()));
        let speaker_repo = Arc::new(DbSpeakerRepository::new(db.clone()));
        let sponsor_repo = Arc::new(DbSponsorRepository::new(db.clone()));
        let user_repo = Arc::new(DbUserRepository::new(db.clone()));
        let venue_repo = Arc::new(DbVenueRepository::new(db.clone()));
        let conference_repo = Arc::new(DbConferenceRepository::new(db.clone()));
        let organization_repo = Arc::new(DbOrganizationRepository::new(db.clone()));

        let user_service = Arc::new(UserServiceImpl::new(user_repo));
        let auth_service = Arc::new(AuthServiceImpl::new(config.clone(), user_service.clone()));
        let client_service = Arc::new(ClientServiceImpl::new(client_repo));
        let exhibitor_service = Arc::new(ExhibitorServiceImpl::new(exhibitor_repo));
        let participant_service = Arc::new(ParticipantServiceImpl::new(participant_repo));
        let registration_service = Arc::new(RegistrationServiceImpl::new(registration_repo));
        let speaker_service = Arc::new(SpeakerServiceImpl::new(speaker_repo));
        let sponsor_service = Arc::new(SponsorServiceImpl::new(sponsor_repo));
        let venue_service = Arc::new(VenueServiceImpl::new(venue_repo.clone()));
        let conference_service =
            Arc::new(ConferenceServiceImpl::new(conference_repo, venue_repo));
        let organization_service = Arc::new(OrganizationServiceImpl::new(organization_repo));

        Ok(Self {
            config,
            db,
            auth_service,
            client_service,
            exhibitor_service,
            participant_service,
            registration_service,
            speaker_service,
            sponsor_service,
            user_service,
            venue_service,
            conference_service,
            organization_service,
        })
    }
}
