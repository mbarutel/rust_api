use std::sync::Arc;

use chrono::Utc;

use crate::{
    application::{
        dto::participant::{CreateParticipantRequest, UpdateParticipantRequest},
        entity::participant::ParticipantEntity,
        error::AppError,
        repository::participant::ParticipantRepository,
        service::participant::ParticipantService,
    },
    domain::models::participant::{Participant, ParticipantRole},
};

pub struct ParticipantServiceImpl {
    participant_repo: Arc<dyn ParticipantRepository>,
}

impl ParticipantServiceImpl {
    pub fn new(participant_repo: Arc<dyn ParticipantRepository>) -> Self {
        Self { participant_repo }
    }
}

#[async_trait::async_trait]
impl ParticipantService for ParticipantServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Participant>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.participant_repo.count().await?;
        let participants = self
            .participant_repo
            .find_all(offset, per_page)
            .await?
            .into_iter()
            .map(Participant::from)
            .collect();
        Ok((participants, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<Participant, AppError> {
        Ok(Participant::from(
            self.participant_repo.find_by_id(id).await?,
        ))
    }

    async fn find_by_registration(
        &self,
        registration_id: u64,
    ) -> Result<Vec<Participant>, AppError> {
        Ok(self
            .participant_repo
            .find_by_registration(registration_id)
            .await?
            .into_iter()
            .map(Participant::from)
            .collect())
    }

    async fn create(&self, dto: CreateParticipantRequest) -> Result<Participant, AppError> {
        let role = dto
            .role
            .as_deref()
            .map(|r| {
                ParticipantRole::try_from(r)
                    .map_err(|_| AppError::Validation(format!("invalid role: {}", r)))
            })
            .transpose()?
            .unwrap_or(ParticipantRole::Delegate);

        let entity = ParticipantEntity {
            id: 0,
            registration_id: dto.registration_id,
            client_id: dto.client_id,
            participant_role: role.as_str().to_string(),
            dietary_requirements: dto.dietary_requirements,
            accessibility_needs: dto.accessibility_needs,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(Participant::from(
            self.participant_repo.create(entity).await?,
        ))
    }

    async fn update(
        &self,
        id: u64,
        dto: UpdateParticipantRequest,
    ) -> Result<Participant, AppError> {
        let entity = self.participant_repo.find_by_id(id).await?;

        let role = dto
            .role
            .as_deref()
            .map(|r| {
                ParticipantRole::try_from(r)
                    .map_err(|_| AppError::Validation(format!("invalid role: {}", r)))
            })
            .transpose()?
            .map(|r| r.as_str().to_string())
            .unwrap_or(entity.participant_role.clone());

        let entity = ParticipantEntity {
            participant_role: role,
            dietary_requirements: dto.dietary_requirements.or(entity.dietary_requirements),
            accessibility_needs: dto.accessibility_needs.or(entity.accessibility_needs),
            updated_at: Utc::now(),
            ..entity
        };
        Ok(Participant::from(
            self.participant_repo.update(entity).await?,
        ))
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.participant_repo.delete(id).await?)
    }
}
