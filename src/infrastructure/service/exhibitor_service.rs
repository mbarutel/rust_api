use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::exhibitor_dto::{CreateExhibitorRequest, UpdateExhibitorRequest},
        entity::exhibitor_entity::ExhibitorEntity,
        error::AppError,
        repository::exhibitor_repository::ExhibitorRepository,
        service::exhibitor_service::ExhibitorService,
    },
    domain::models::exhibitor::Exhibitor,
};

pub struct ExhibitorServiceImpl {
    exhibitor_repo: Arc<dyn ExhibitorRepository>,
}

impl ExhibitorServiceImpl {
    pub fn new(exhibitor_repo: Arc<dyn ExhibitorRepository>) -> Self {
        Self { exhibitor_repo }
    }
}

#[async_trait::async_trait]
impl ExhibitorService for ExhibitorServiceImpl {
    async fn find_by_participant_id(&self, participant_id: u64) -> Result<Exhibitor, AppError> {
        Ok(Exhibitor::from(
            self.exhibitor_repo
                .find_by_participant_id(participant_id)
                .await?,
        ))
    }

    async fn create(
        &self,
        participant_id: u64,
        dto: CreateExhibitorRequest,
    ) -> Result<Exhibitor, AppError> {
        let entity = ExhibitorEntity {
            id: 0,
            participant_id,
            company_name: dto.company_name,
            power_required: dto.power_required.unwrap_or(false) as i8,
            internet_required: dto.internet_required.unwrap_or(false) as i8,
            notes_internal: dto.notes_internal,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(Exhibitor::from(self.exhibitor_repo.create(entity).await?))
    }

    async fn update(
        &self,
        participant_id: u64,
        dto: UpdateExhibitorRequest,
    ) -> Result<Exhibitor, AppError> {
        let entity = self
            .exhibitor_repo
            .find_by_participant_id(participant_id)
            .await?;
        let entity = ExhibitorEntity {
            company_name: dto.company_name.unwrap_or(entity.company_name),
            power_required: dto
                .power_required
                .map(|v| v as i8)
                .unwrap_or(entity.power_required),
            internet_required: dto
                .internet_required
                .map(|v| v as i8)
                .unwrap_or(entity.internet_required),
            notes_internal: dto.notes_internal.or(entity.notes_internal),
            updated_at: Utc::now(),
            ..entity
        };
        Ok(Exhibitor::from(self.exhibitor_repo.update(entity).await?))
    }

    async fn delete(&self, participant_id: u64) -> Result<(), AppError> {
        let entity = self
            .exhibitor_repo
            .find_by_participant_id(participant_id)
            .await?;
        Ok(self.exhibitor_repo.delete(entity.id).await?)
    }
}
