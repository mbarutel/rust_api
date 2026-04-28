use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::MySqlPool;
use std::sync::Arc;

use crate::application::{
    dto::registration_dto::{RegisterDelegateRequest, RegistrationResponse},
    entity::{
        client_entity::ClientEntity, organization_entity::OrganizationEntity,
        participant_entity::ParticipantEntity, registration_entity::RegistrationEntity,
    },
    error::AppError,
    repository::{
        client_repository::ClientRepository, organization_repository::OrganizationRepository,
        participant_repository::ParticipantRepository,
        registration_repository::RegistrationRepository,
    },
    service::conference_registration_service::ConferenceRegistrationService,
};

use crate::domain::{
    error::DomainError,
    models::registration::{Registration, RegistrationStatus},
};

pub struct ConferenceRegistrationServiceImpl {
    pool: MySqlPool,
    organization_repo: Arc<dyn OrganizationRepository>,
    client_repo: Arc<dyn ClientRepository>,
    registration_repo: Arc<dyn RegistrationRepository>,
    participant_repo: Arc<dyn ParticipantRepository>,
}

impl ConferenceRegistrationServiceImpl {
    pub fn new(
        pool: MySqlPool,
        organization_repo: Arc<dyn OrganizationRepository>,
        client_repo: Arc<dyn ClientRepository>,
        registration_repo: Arc<dyn RegistrationRepository>,
        participant_repo: Arc<dyn ParticipantRepository>,
    ) -> Self {
        Self {
            pool,
            organization_repo,
            client_repo,
            registration_repo,
            participant_repo,
        }
    }
}

#[async_trait::async_trait]
impl ConferenceRegistrationService for ConferenceRegistrationServiceImpl {
    async fn register_delegates(
        &self,
        dto: RegisterDelegateRequest,
    ) -> Result<RegistrationResponse, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Domain(DomainError::Database(e.to_string())))?;

        let registration = self
            .registration_repo
            .create_in_tx(
                &mut tx,
                RegistrationEntity {
                    id: 0,
                    conference_id: dto.conference_id,
                    status: RegistrationStatus::Submitted.as_str().to_string(),
                    cost: dto.price_tier.price,
                    discount_code: dto.discount_code.clone(),
                    discount_amount: Decimal::ZERO,
                    amount_paid: Decimal::ZERO,
                    created_by_id: None,
                    notes_internal: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            )
            .await?;

        for delegate in dto.delegates {
            let org = self
                .organization_repo
                .create_in_tx(
                    &mut tx,
                    OrganizationEntity {
                        id: 0,
                        name: delegate.organization_name.clone(),
                        website: None,
                        phone: None,
                        billing_email: delegate.email.clone(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    },
                )
                .await?;

            let client = self
                .client_repo
                .create_in_tx(
                    &mut tx,
                    ClientEntity {
                        id: 0,
                        organization_id: Some(org.id), // This should be an Option as a client should belong to an organization
                        first_name: delegate.first_name,
                        last_name: delegate.last_name,
                        email: delegate.email,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    },
                )
                .await?;

            self.participant_repo
                .create_in_tx(
                    &mut tx,
                    ParticipantEntity {
                        id: 0,
                        registration_id: registration.id,
                        client_id: client.id,
                        participant_role: "delegate".to_string(),
                        dietary_requirements: Some(delegate.dietary_requirements),
                        accessibility_needs: None,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    },
                )
                .await?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::Domain(DomainError::Database(e.to_string())))?;

        Ok(RegistrationResponse::from(Registration::from(registration)))
    }
}
