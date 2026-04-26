use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::conference_dto::{CreateConferenceRequest, UpdateConferenceRequest},
        entity::conference_entity::ConferenceEntity,
        error::AppError,
        repository::{
            conference_repository::ConferenceRepository, venue_repository::VenueRepository,
        },
        service::conference_service::ConferenceService,
    },
    domain::{error::DomainError, models::conference::Conference},
};

pub struct ConferenceServiceImpl {
    conference_repo: Arc<dyn ConferenceRepository>,
    venue_repo: Arc<dyn VenueRepository>,
}

impl ConferenceServiceImpl {
    pub fn new(
        conference_repo: Arc<dyn ConferenceRepository>,
        venue_repo: Arc<dyn VenueRepository>,
    ) -> Self {
        Self {
            conference_repo,
            venue_repo,
        }
    }
}

#[async_trait::async_trait]
impl ConferenceService for ConferenceServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Conference>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.conference_repo.count().await?;
        let entities = self.conference_repo.find_all(offset, per_page).await?;
        let mut conferences = Vec::with_capacity(entities.len());

        for entity in entities {
            let venue = match entity.venue_id {
                Some(id) => match self.venue_repo.find_by_id(id).await {
                    Ok(v) => Some(v),
                    Err(DomainError::NotFound) => None,
                    Err(e) => return Err(AppError::Domain(e)),
                },
                None => None,
            };

            conferences.push(Conference::from((entity, venue)));
        }

        Ok((conferences, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<Conference, AppError> {
        let conference = self.conference_repo.find_by_id(id).await?;
        let venue = match conference.venue_id {
            Some(id) => match self.venue_repo.find_by_id(id).await {
                Ok(v) => Some(v),
                Err(DomainError::NotFound) => None,
                Err(e) => return Err(AppError::Domain(e)),
            },
            None => None,
        };

        Ok(Conference::from((conference, venue)))
    }

    async fn create(&self, dto: CreateConferenceRequest) -> Result<Conference, AppError> {
        let now = Utc::now();

        let conference_entity = ConferenceEntity {
            id: 0,
            code: dto.code,
            name: dto.name,
            poster_url: dto.poster_url,
            description: dto.description,
            start_date: dto.start_date,
            end_date: dto.end_date,
            venue_id: None,
            published: 0,
            created_at: now,
            updated_at: now,
        };

        let conference_entity = self.conference_repo.create(conference_entity).await?;

        Ok(Conference::from((conference_entity, None)))
    }

    async fn update(&self, id: u64, dto: UpdateConferenceRequest) -> Result<Conference, AppError> {
        let now = Utc::now();
        let conference_entity = self.conference_repo.find_by_id(id).await?;
        let conference_entity = ConferenceEntity {
            id: conference_entity.id,
            code: conference_entity.code,
            name: dto.name.unwrap_or(conference_entity.name),
            poster_url: dto.poster_url.or(conference_entity.poster_url),
            description: dto.description.or(conference_entity.description),
            start_date: dto.start_date.or(conference_entity.start_date),
            end_date: dto.end_date.or(conference_entity.end_date),
            venue_id: dto.venue_id.or(conference_entity.venue_id),
            published: conference_entity.published,
            created_at: conference_entity.created_at,
            updated_at: now,
        };

        let conference_entity = self.conference_repo.update(conference_entity).await?;

        let venue_entity = match conference_entity.venue_id {
            Some(id) => match self.venue_repo.find_by_id(id).await {
                Ok(v) => Some(v),
                Err(DomainError::NotFound) => None,
                Err(e) => return Err(AppError::Domain(e)),
            },
            None => None,
        };

        Ok(Conference::from((conference_entity, venue_entity)))
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.conference_repo.delete(id).await?)
    }

    async fn publish(&self, id: u64, published: bool) -> Result<Conference, AppError> {
        let entity = self.conference_repo.find_by_id(id).await?;
        let entity = ConferenceEntity {
            published: published as i8,
            updated_at: Utc::now(),
            ..entity
        };
        let entity = self.conference_repo.update(entity).await?;

        let venue = match entity.venue_id {
            Some(vid) => match self.venue_repo.find_by_id(vid).await {
                Ok(v) => Some(v),
                Err(DomainError::NotFound) => None,
                Err(e) => return Err(AppError::Domain(e)),
            },
            None => None,
        };

        Ok(Conference::from((entity, venue)))
    }
}
