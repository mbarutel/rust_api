use std::sync::Arc;

use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::MySqlPool;

use crate::{
    application::{
        dto::{
            ConferenceResponse, PriceTierResponse, RegistrationResponse,
            delegate_registration::{DelegateFormResponse, DelegateRegistrationRequest},
        },
        entity::{ClientEntity, OrganizationEntity, ParticipantEntity, RegistrationEntity},
        error::AppError,
        repository::{
            ClientRepository, ConferenceRepository, GroupDiscountRepository,
            OrganizationRepository, ParticipantRepository, PriceTierRepository,
            RegistrationRepository, VenueRepository,
        },
        service::ConferenceRegistrationService,
    },
    domain::{
        error::DomainError,
        models::{
            Conference, ParticipantRole, PriceTier,
            registration::{Registration, RegistrationStatus},
        },
    },
};

pub struct ConferenceRegistrationServiceImpl {
    pool: MySqlPool,
    organization_repo: Arc<dyn OrganizationRepository>,
    client_repo: Arc<dyn ClientRepository>,
    registration_repo: Arc<dyn RegistrationRepository>,
    participant_repo: Arc<dyn ParticipantRepository>,
    conference_repo: Arc<dyn ConferenceRepository>,
    venue_repo: Arc<dyn VenueRepository>,
    price_tier_repo: Arc<dyn PriceTierRepository>,
    group_discount_repo: Arc<dyn GroupDiscountRepository>,
}

impl ConferenceRegistrationServiceImpl {
    pub fn new(
        pool: MySqlPool,
        organization_repo: Arc<dyn OrganizationRepository>,
        client_repo: Arc<dyn ClientRepository>,
        registration_repo: Arc<dyn RegistrationRepository>,
        participant_repo: Arc<dyn ParticipantRepository>,
        conference_repo: Arc<dyn ConferenceRepository>,
        venue_repo: Arc<dyn VenueRepository>,
        price_tier_repo: Arc<dyn PriceTierRepository>,
        group_discount_repo: Arc<dyn GroupDiscountRepository>,
    ) -> Self {
        Self {
            pool,
            organization_repo,
            client_repo,
            registration_repo,
            participant_repo,
            conference_repo,
            venue_repo,
            price_tier_repo,
            group_discount_repo,
        }
    }
}

#[async_trait::async_trait]
impl ConferenceRegistrationService for ConferenceRegistrationServiceImpl {
    async fn register_delegates_form(
        &self,
        conference_id: u64,
    ) -> Result<DelegateFormResponse, AppError> {
        let conference = self.conference_repo.find_by_id(conference_id).await?;

        if conference.start_date.is_none() {
            return Err(AppError::Domain(DomainError::InvalidTransition(
                "Registration is not ready for conferences without a start date".to_string(),
            )));
        }

        let venue = match conference.venue_id {
            Some(id) => Some(self.venue_repo.find_by_id(id).await?),
            None => None,
        };

        let price_tiers = self
            .price_tier_repo
            .find_by_conference_id(conference.id)
            .await?
            .into_iter()
            .map(PriceTier::from)
            .map(PriceTierResponse::from)
            .collect();

        Ok(DelegateFormResponse {
            conference: ConferenceResponse::from(Conference::from(conference).with_venue(venue)),
            price_tiers,
        })
    }

    async fn register_delegates(
        &self,
        dto: DelegateRegistrationRequest,
    ) -> Result<RegistrationResponse, AppError> {
        // Fetch and validate conference
        let conference = self.conference_repo.find_by_id(dto.conference_id).await?;
        if !conference.is_published() {
            return Err(AppError::Validation(
                "Registrations are not open for this conference".to_string(),
            ));
        }

        // Fetch and validate price tier belonging to this conference
        let price_tier = PriceTier::from(self.price_tier_repo.find_by_id(dto.price_tier_id).await?);
        if price_tier.conference_id != dto.conference_id {
            return Err(AppError::Validation(
                "The selected price tier does not belong to this conference.".to_string(),
            ));
        }
        if price_tier.is_expired() {
            return Err(AppError::Validation(
                "The selected price tier has expired. Please select a valid one.".to_string(),
            ));
        }

        // Validate and apply group discount (if provided)
        let delegate_count = dto.delegates.len();
        let mut discount_amount = Decimal::ZERO;
        let mut discount_code: Option<String> = None;

        if let Some(ref code) = dto.group_discount_code {
            let group_discount = self
                .group_discount_repo
                .find_by_code(code)
                .await?
                .ok_or_else(|| AppError::Validation("Invalid group discount code.".to_string()))?;

            // Verify the discount belongs to this conference
            if conference.group_discount_id != Some(group_discount.id) {
                return Err(AppError::Validation(
                    "This discount code is not valid for this conference.".to_string(),
                ));
            }

            if !group_discount.is_active() {
                return Err(AppError::Validation(
                    "This discount code is no longer active.".to_string(),
                ));
            }
            if let Some(valid_until) = group_discount.valid_until {
                let now = Utc::now().naive_utc();
                if now > valid_until {
                    return Err(AppError::Validation(
                        "This discount code has expired".to_string(),
                    ));
                }
            }
            if delegate_count < group_discount.min_quantity as usize {
                return Err(AppError::Validation(format!(
                    "A minimum of {} delegates is required to use this discount.",
                    group_discount.min_quantity
                )));
            }

            discount_amount = Decimal::from(group_discount.free_quantity) * price_tier.price;
            discount_code = Some(code.clone());
        }

        // Calculate total cost
        let total_cost = price_tier.price * Decimal::from(delegate_count as u64) - discount_amount;

        // Begin transaction
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Domain(DomainError::Database(e.to_string())))?;

        // Process first delegate to get created_by_id
        let mut delegates_iter = dto.delegates.into_iter();
        let first_delegate = delegates_iter
            .next()
            .ok_or_else(|| AppError::Validation("Delegates is an empty list".to_string()))?;

        let first_org = match self
            .organization_repo
            .find_by_name(&first_delegate.organization_name)
            .await?
        {
            Some(existing) => existing,
            None => {
                self.organization_repo
                    .create_in_tx(
                        &mut tx,
                        OrganizationEntity {
                            id: 0,
                            name: first_delegate.organization_name.clone(),
                            website: None,
                            phone: None,
                            billing_email: first_delegate.email.clone(),
                            created_at: Utc::now(),
                            updated_at: Utc::now(),
                        },
                    )
                    .await?
            }
        };

        let first_client = match self
            .client_repo
            .find_by_email(&first_delegate.email)
            .await?
        {
            Some(existing) => existing,
            None => {
                self.client_repo
                    .create_in_tx(
                        &mut tx,
                        ClientEntity {
                            id: 0,
                            organization_id: Some(first_org.id),
                            first_name: first_delegate.first_name,
                            last_name: first_delegate.last_name,
                            email: first_delegate.email.clone(),
                            created_at: Utc::now(),
                            updated_at: Utc::now(),
                        },
                    )
                    .await?
            }
        };

        // Create registration with correct cost, discount, creator, and referrer
        let registration = self
            .registration_repo
            .create_in_tx(
                &mut tx,
                RegistrationEntity {
                    id: 0,
                    conference_id: dto.conference_id,
                    status: RegistrationStatus::Submitted.as_str().to_string(),
                    cost: total_cost,
                    discount_code,
                    discount_amount,
                    amount_paid: Decimal::ZERO,
                    created_by_id: first_client.id,
                    notes_internal: None,
                    referrer: Some(dto.referrer),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            )
            .await?;

        // Create participant for first delegate
        self.participant_repo
            .create_in_tx(
                &mut tx,
                ParticipantEntity {
                    id: 0,
                    registration_id: registration.id,
                    client_id: first_client.id,
                    participant_role: String::from(ParticipantRole::Delegate.as_str()),
                    dietary_requirements: Some(first_delegate.dietary_requirements),
                    accessibility_needs: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            )
            .await?;

        // Process the remaining delegates
        for delegate in delegates_iter {
            let org = match self
                .organization_repo
                .find_by_name(&delegate.organization_name)
                .await?
            {
                Some(existing) => existing,
                None => {
                    self.organization_repo
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
                        .await?
                }
            };

            let client = match self.client_repo.find_by_email(&delegate.email).await? {
                Some(existing) => existing,
                None => {
                    self.client_repo
                        .create_in_tx(
                            &mut tx,
                            ClientEntity {
                                id: 0,
                                organization_id: Some(org.id),
                                first_name: delegate.first_name,
                                last_name: delegate.last_name,
                                email: delegate.email.clone(),
                                created_at: Utc::now(),
                                updated_at: Utc::now(),
                            },
                        )
                        .await?
                }
            };

            self.participant_repo
                .create_in_tx(
                    &mut tx,
                    ParticipantEntity {
                        id: 0,
                        registration_id: registration.id,
                        client_id: client.id,
                        participant_role: String::from(ParticipantRole::Delegate.as_str()),
                        dietary_requirements: Some(delegate.dietary_requirements),
                        accessibility_needs: None,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    },
                )
                .await?;
        }

        // Commit
        tx.commit()
            .await
            .map_err(|e| AppError::Domain(DomainError::Database(e.to_string())))?;

        Ok(RegistrationResponse::from(Registration::from(registration)))
    }
}
