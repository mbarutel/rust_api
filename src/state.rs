use std::sync::Arc;

use sqlx::mysql::MySqlPool;

use crate::{
    application::{
        repository::{
            activity::ActivityRepository,
            activity_booking::ActivityBookingRepository,
            client::ClientRepository,
            conference::ConferenceRepository,
            exhibitor::ExhibitorRepository,
            masterclass::{MasterclassInstructorRepository, MasterclassRepository},
            masterclass_booking::MasterclassBookingRepository,
            organization::OrganizationRepository,
            participant::ParticipantRepository,
            price_tier::PriceTierRepository,
            registration::RegistrationRepository,
            speaker::SpeakerRepository,
            sponsor::SponsorRepository,
            user::UserRepository,
            venue::VenueRepository,
        },
        service::{
            activity::ActivityService, auth::AuthService, client::ClientService,
            conference::ConferenceService, conference_registration::ConferenceRegistrationService,
            exhibitor::ExhibitorService, masterclass::MasterclassService,
            organization::OrganizationService, participant::ParticipantService,
            registration::RegistrationService, speaker::SpeakerService, sponsor::SponsorService,
            user::UserService, venue::VenueService,
        },
    },
    infrastructure::{
        config::Config,
        database::{
            pool::create_pool,
            repository::{
                activity::DbActivityRepository,
                activity_booking::DbActivityBookingRepository,
                client::DbClientRepository,
                conference::DbConferenceRepository,
                exhibitor::DbExhibitorRepository,
                masterclass::{DbMasterclassInstructorRepository, DbMasterclassRepository},
                masterclass_booking::DbMasterclassBookingRepository,
                organization::DbOrganizationRepository,
                participant::DbParticipantRepository,
                price_tier::DbPriceTierRepository,
                registration::DbRegistrationRepository,
                speaker::DbSpeakerRepository,
                sponsor::DbSponsorRepository,
                user::DbUserRepository,
                venue::DbVenueRepository,
            },
        },
        service::{
            activity::ActivityServiceImpl, auth::AuthServiceImpl, client::ClientServiceImpl,
            conference::ConferenceServiceImpl,
            conference_registration::ConferenceRegistrationServiceImpl,
            exhibitor::ExhibitorServiceImpl, masterclass::MasterclassServiceImpl,
            organization::OrganizationServiceImpl, participant::ParticipantServiceImpl,
            registration::RegistrationServiceImpl, speaker::SpeakerServiceImpl,
            sponsor::SponsorServiceImpl, user::UserServiceImpl, venue::VenueServiceImpl,
        },
    },
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: MySqlPool,
    pub services: Services,
}

impl AppState {
    pub async fn init(config: Arc<Config>) -> anyhow::Result<Self> {
        let db = create_pool(&config.database_url).await?;
        let repos = Repositories::build(&db);
        let services = Services::build(&db, config.clone(), repos);

        Ok(Self {
            config,
            db,
            services,
        })
    }
}

struct Repositories {
    activity_booking: Arc<dyn ActivityBookingRepository>,
    activity: Arc<dyn ActivityRepository>,
    client: Arc<dyn ClientRepository>,
    exhibitor: Arc<dyn ExhibitorRepository>,
    masterclass_booking: Arc<dyn MasterclassBookingRepository>,
    masterclass: Arc<dyn MasterclassRepository>,
    masterclass_instructor: Arc<dyn MasterclassInstructorRepository>,
    participant: Arc<dyn ParticipantRepository>,
    registration: Arc<dyn RegistrationRepository>,
    speaker: Arc<dyn SpeakerRepository>,
    sponsor: Arc<dyn SponsorRepository>,
    user: Arc<dyn UserRepository>,
    venue: Arc<dyn VenueRepository>,
    conference: Arc<dyn ConferenceRepository>,
    organization: Arc<dyn OrganizationRepository>,
    price_tier: Arc<dyn PriceTierRepository>,
}

impl Repositories {
    fn build(db: &MySqlPool) -> Self {
        Self {
            activity_booking: Arc::new(DbActivityBookingRepository::new(db.clone())),
            activity: Arc::new(DbActivityRepository::new(db.clone())),
            client: Arc::new(DbClientRepository::new(db.clone())),
            exhibitor: Arc::new(DbExhibitorRepository::new(db.clone())),
            masterclass_booking: Arc::new(DbMasterclassBookingRepository::new(db.clone())),
            masterclass: Arc::new(DbMasterclassRepository::new(db.clone())),
            masterclass_instructor: Arc::new(DbMasterclassInstructorRepository::new(db.clone())),
            participant: Arc::new(DbParticipantRepository::new(db.clone())),
            registration: Arc::new(DbRegistrationRepository::new(db.clone())),
            speaker: Arc::new(DbSpeakerRepository::new(db.clone())),
            sponsor: Arc::new(DbSponsorRepository::new(db.clone())),
            user: Arc::new(DbUserRepository::new(db.clone())),
            venue: Arc::new(DbVenueRepository::new(db.clone())),
            conference: Arc::new(DbConferenceRepository::new(db.clone())),
            organization: Arc::new(DbOrganizationRepository::new(db.clone())),
            price_tier: Arc::new(DbPriceTierRepository::new(db.clone())),
        }
    }
}

#[derive(Clone)]
pub struct Services {
    pub activity: Arc<dyn ActivityService>,
    pub auth: Arc<dyn AuthService>,
    pub client: Arc<dyn ClientService>,
    pub conference: Arc<dyn ConferenceService>,
    pub exhibitor: Arc<dyn ExhibitorService>,
    pub masterclass: Arc<dyn MasterclassService>,
    pub organization: Arc<dyn OrganizationService>,
    pub participant: Arc<dyn ParticipantService>,
    pub registration: Arc<dyn RegistrationService>,
    pub speaker: Arc<dyn SpeakerService>,
    pub sponsor: Arc<dyn SponsorService>,
    pub user: Arc<dyn UserService>,
    pub venue: Arc<dyn VenueService>,
    pub conference_registration: Arc<dyn ConferenceRegistrationService>,
}

impl Services {
    fn build(db: &MySqlPool, config: Arc<Config>, repos: Repositories) -> Self {
        let user = Arc::new(UserServiceImpl::new(repos.user.clone()));
        let auth = Arc::new(AuthServiceImpl::new(config.clone(), user.clone()));
        let activity = Arc::new(ActivityServiceImpl::new(
            repos.activity.clone(),
            repos.activity_booking.clone(),
        ));
        let client = Arc::new(ClientServiceImpl::new(repos.client.clone()));
        let exhibitor = Arc::new(ExhibitorServiceImpl::new(repos.exhibitor.clone()));
        let masterclass = Arc::new(MasterclassServiceImpl::new(
            repos.masterclass.clone(),
            repos.masterclass_instructor.clone(),
            repos.masterclass_booking.clone(),
        ));
        let participant = Arc::new(ParticipantServiceImpl::new(repos.participant.clone()));
        let registration = Arc::new(RegistrationServiceImpl::new(repos.registration.clone()));
        let speaker = Arc::new(SpeakerServiceImpl::new(repos.speaker));
        let sponsor = Arc::new(SponsorServiceImpl::new(repos.sponsor));
        let venue = Arc::new(VenueServiceImpl::new(repos.venue.clone()));
        let conference = Arc::new(ConferenceServiceImpl::new(
            db.clone(),
            repos.conference.clone(),
            repos.venue.clone(),
            repos.price_tier.clone(),
        ));
        let organization = Arc::new(OrganizationServiceImpl::new(repos.organization.clone()));
        let conference_registration = Arc::new(ConferenceRegistrationServiceImpl::new(
            db.clone(),
            repos.organization.clone(),
            repos.client.clone(),
            repos.registration.clone(),
            repos.participant.clone(),
        ));

        Self {
            user,
            auth,
            activity,
            client,
            exhibitor,
            masterclass,
            participant,
            registration,
            speaker,
            sponsor,
            venue,
            conference,
            organization,
            conference_registration,
        }
    }
}
