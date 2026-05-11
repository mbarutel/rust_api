use std::sync::Arc;

use chrono::Utc;
use rust_decimal::Decimal;

use crate::{
    application::{
        dto::registration::{
            CreateRegistrationRequest, RecordPaymentRequest, TransitionStatusRequest,
            UpdateRegistrationRequest,
        },
        entity::registration::RegistrationEntity,
        error::AppError,
        repository::registration::RegistrationRepository,
        service::registration::RegistrationService,
    },
    domain::{
        error::DomainError,
        models::registration::{Registration, RegistrationStatus},
    },
};

pub struct RegistrationServiceImpl {
    registration_repo: Arc<dyn RegistrationRepository>,
}

impl RegistrationServiceImpl {
    pub fn new(registration_repo: Arc<dyn RegistrationRepository>) -> Self {
        Self { registration_repo }
    }
}

#[async_trait::async_trait]
impl RegistrationService for RegistrationServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Registration>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.registration_repo.count().await?;
        let registrations = self
            .registration_repo
            .find_all(offset, per_page)
            .await?
            .into_iter()
            .map(Registration::from)
            .collect();
        Ok((registrations, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<Registration, AppError> {
        Ok(Registration::from(
            self.registration_repo.find_by_id(id).await?,
        ))
    }

    async fn create(&self, dto: CreateRegistrationRequest) -> Result<Registration, AppError> {
        let entity = RegistrationEntity {
            id: 0,
            conference_id: dto.conference_id,
            status: RegistrationStatus::Submitted.as_str().to_string(),
            cost: dto.cost.unwrap_or(Decimal::ZERO),
            discount_code: dto.discount_code,
            discount_amount: dto.discount_amount.unwrap_or(Decimal::ZERO),
            amount_paid: Decimal::ZERO,
            created_by_id: dto.created_by_id,
            notes_internal: dto.notes_internal,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(Registration::from(
            self.registration_repo.create(entity).await?,
        ))
    }

    async fn update(
        &self,
        id: u64,
        dto: UpdateRegistrationRequest,
    ) -> Result<Registration, AppError> {
        let entity = self.registration_repo.find_by_id(id).await?;
        let entity = RegistrationEntity {
            cost: dto.cost.unwrap_or(entity.cost),
            discount_code: dto.discount_code.or(entity.discount_code),
            discount_amount: dto.discount_amount.unwrap_or(entity.discount_amount),
            notes_internal: dto.notes_internal.or(entity.notes_internal),
            updated_at: Utc::now(),
            ..entity
        };
        Ok(Registration::from(
            self.registration_repo.update(entity).await?,
        ))
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.registration_repo.delete(id).await?)
    }

    async fn transition_status(
        &self,
        id: u64,
        dto: TransitionStatusRequest,
    ) -> Result<Registration, AppError> {
        let next = RegistrationStatus::try_from(dto.status.as_str())
            .map_err(|_| AppError::Validation(format!("unknown status: {}", dto.status)))?;

        let entity = self.registration_repo.find_by_id(id).await?;
        let current = RegistrationStatus::try_from(entity.status.as_str())
            .unwrap_or(RegistrationStatus::Submitted);

        if !current.can_transition_to(&next) {
            return Err(AppError::Domain(DomainError::InvalidTransition(format!(
                "cannot transition from '{}' to '{}'",
                current.as_str(),
                next.as_str()
            ))));
        }

        let entity = RegistrationEntity {
            status: next.as_str().to_string(),
            updated_at: Utc::now(),
            ..entity
        };
        Ok(Registration::from(
            self.registration_repo.update(entity).await?,
        ))
    }

    async fn record_payment(
        &self,
        id: u64,
        dto: RecordPaymentRequest,
    ) -> Result<Registration, AppError> {
        let entity = self.registration_repo.find_by_id(id).await?;
        let entity = RegistrationEntity {
            amount_paid: entity.amount_paid + dto.amount,
            updated_at: Utc::now(),
            ..entity
        };
        Ok(Registration::from(
            self.registration_repo.update(entity).await?,
        ))
    }
}
