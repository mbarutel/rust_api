use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::speaker_dto::{CreateSpeakerRequest, UpdateSpeakerRequest},
        entity::speaker_entity::SpeakerEntity,
        error::AppError,
        repository::speaker_repository::SpeakerRepository,
        service::speaker_service::SpeakerService,
    },
    domain::models::speaker::Speaker,
};

pub struct SpeakerServiceImpl {
    speaker_repo: Arc<dyn SpeakerRepository>,
}

impl SpeakerServiceImpl {
    pub fn new(speaker_repo: Arc<dyn SpeakerRepository>) -> Self {
        Self { speaker_repo }
    }
}

#[async_trait::async_trait]
impl SpeakerService for SpeakerServiceImpl {
    async fn find_by_participant_id(&self, participant_id: u64) -> Result<Speaker, AppError> {
        Ok(Speaker::from(
            self.speaker_repo
                .find_by_participant_id(participant_id)
                .await?,
        ))
    }

    async fn create(
        &self,
        participant_id: u64,
        dto: CreateSpeakerRequest,
    ) -> Result<Speaker, AppError> {
        let entity = SpeakerEntity {
            id: 0,
            participant_id,
            talk_title: dto.talk_title,
            talk_abstract: dto.talk_abstract,
            duration_minutes: dto.duration_minutes,
            av_requirements: dto.av_requirements,
            headshot: dto.headshot,
            bio: dto.bio,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(Speaker::from(self.speaker_repo.create(entity).await?))
    }

    async fn update(
        &self,
        participant_id: u64,
        dto: UpdateSpeakerRequest,
    ) -> Result<Speaker, AppError> {
        let entity = self
            .speaker_repo
            .find_by_participant_id(participant_id)
            .await?;
        let entity = SpeakerEntity {
            talk_title: dto.talk_title.unwrap_or(entity.talk_title),
            talk_abstract: dto.talk_abstract.or(entity.talk_abstract),
            duration_minutes: dto.duration_minutes.or(entity.duration_minutes),
            av_requirements: dto.av_requirements.or(entity.av_requirements),
            headshot: dto.headshot.or(entity.headshot),
            bio: dto.bio.or(entity.bio),
            updated_at: Utc::now(),
            ..entity
        };
        Ok(Speaker::from(self.speaker_repo.update(entity).await?))
    }

    async fn delete(&self, participant_id: u64) -> Result<(), AppError> {
        let entity = self
            .speaker_repo
            .find_by_participant_id(participant_id)
            .await?;
        Ok(self.speaker_repo.delete(entity.id).await?)
    }
}
