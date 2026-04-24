use crate::{
    application::service::{
        auth_service::AuthService, conference_service::ConferenceService,
        organization_service::OrganizationService, user_service::UserService,
        venue_service::VenueService,
    },
    infrastructure::{
        config::Config,
        database::{
            pool::create_pool,
            repository::{
                conference_repository::DbConferenceRepository,
                organization_repository::DbOrganizationRepository,
                user_repository::DbUserRepository,
                venue_repository::DbVenueRepository,
            },
        },
        service::{
            auth_service::AuthServiceImpl, conference_service::ConferenceServiceImpl,
            organization_service::OrganizationServiceImpl, user_service::UserServiceImpl,
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
    pub user_service: Arc<dyn UserService>,
    pub venue_service: Arc<dyn VenueService>,
    pub conference_service: Arc<dyn ConferenceService>,
    pub organization_service: Arc<dyn OrganizationService>,
}

impl AppState {
    pub async fn init(config: Arc<Config>) -> anyhow::Result<Self> {
        let db = create_pool(&config.database_url).await?;

        let user_repo = Arc::new(DbUserRepository::new(db.clone()));
        let venue_repo = Arc::new(DbVenueRepository::new(db.clone()));
        let conference_repo = Arc::new(DbConferenceRepository::new(db.clone()));
        let organization_repo = Arc::new(DbOrganizationRepository::new(db.clone()));

        let user_service = Arc::new(UserServiceImpl::new(user_repo));
        let auth_service = Arc::new(AuthServiceImpl::new(config.clone(), user_service.clone()));
        let venue_service = Arc::new(VenueServiceImpl::new(venue_repo.clone()));
        let conference_service =
            Arc::new(ConferenceServiceImpl::new(conference_repo, venue_repo));
        let organization_service = Arc::new(OrganizationServiceImpl::new(organization_repo));

        Ok(Self {
            config,
            db,
            auth_service,
            user_service,
            venue_service,
            conference_service,
            organization_service,
        })
    }
}
