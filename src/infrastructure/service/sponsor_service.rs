use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::sponsor_dto::{CreateSponsorRequest, UpdateSponsorRequest},
        entity::sponsor_entity::SponsorEntity,
        error::AppError,
        repository::sponsor_repository::SponsorRepository,
        service::sponsor_service::SponsorService,
    },
    domain::models::sponsor::Sponsor,
};

pub struct SponsorServiceImpl {
    sponsor_repo: Arc<dyn SponsorRepository>,
}

impl SponsorServiceImpl {
    pub fn new(sponsor_repo: Arc<dyn SponsorRepository>) -> Self {
        Self { sponsor_repo }
    }
}

#[async_trait::async_trait]
impl SponsorService for SponsorServiceImpl {
    async fn find_by_participant_id(&self, participant_id: u64) -> Result<Sponsor, AppError> {
        Ok(Sponsor::from(
            self.sponsor_repo
                .find_by_participant_id(participant_id)
                .await?,
        ))
    }

    async fn create(
        &self,
        participant_id: u64,
        dto: CreateSponsorRequest,
    ) -> Result<Sponsor, AppError> {
        let entity = SponsorEntity {
            id: 0,
            participant_id,
            tier: dto.tier,
            company_name: dto.company_name,
            logo_url: dto.logo_url,
            invoice_contact: dto.invoice_contact,
            benefits_notes: dto.benefits_notes,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(Sponsor::from(self.sponsor_repo.create(entity).await?))
    }

    async fn update(
        &self,
        participant_id: u64,
        dto: UpdateSponsorRequest,
    ) -> Result<Sponsor, AppError> {
        let entity = self
            .sponsor_repo
            .find_by_participant_id(participant_id)
            .await?;
        let entity = SponsorEntity {
            tier: dto.tier.unwrap_or(entity.tier),
            company_name: dto.company_name.or(entity.company_name),
            logo_url: dto.logo_url.or(entity.logo_url),
            invoice_contact: dto.invoice_contact.or(entity.invoice_contact),
            benefits_notes: dto.benefits_notes.or(entity.benefits_notes),
            updated_at: Utc::now(),
            ..entity
        };
        Ok(Sponsor::from(self.sponsor_repo.update(entity).await?))
    }

    async fn delete(&self, participant_id: u64) -> Result<(), AppError> {
        let entity = self
            .sponsor_repo
            .find_by_participant_id(participant_id)
            .await?;
        Ok(self.sponsor_repo.delete(entity.id).await?)
    }
}
