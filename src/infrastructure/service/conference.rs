use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use sqlx::MySqlPool;

use crate::{
    application::{
        dto::conference::{CreateConferenceRequest, UpdateConferenceRequest},
        entity::{conference::ConferenceEntity, price_tier::PriceTierEntity, venue::VenueEntity},
        error::AppError,
        repository::{
            conference::ConferenceRepository, price_tier::PriceTierRepository,
            venue::VenueRepository,
        },
        service::conference::ConferenceService,
    },
    domain::{
        error::DomainError,
        models::{PriceTier, conference::Conference},
        utils::generate_price_tiers,
    },
};

pub struct ConferenceServiceImpl {
    pool: MySqlPool,
    conference_repo: Arc<dyn ConferenceRepository>,
    venue_repo: Arc<dyn VenueRepository>,
    price_tier_repo: Arc<dyn PriceTierRepository>,
}

impl ConferenceServiceImpl {
    pub fn new(
        pool: MySqlPool,
        conference_repo: Arc<dyn ConferenceRepository>,
        venue_repo: Arc<dyn VenueRepository>,
        price_tier_repo: Arc<dyn PriceTierRepository>,
    ) -> Self {
        Self {
            pool,
            conference_repo,
            venue_repo,
            price_tier_repo,
        }
    }
}

#[async_trait::async_trait]
impl ConferenceService for ConferenceServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Conference>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.conference_repo.count().await?;
        let entities = self.conference_repo.find_all(offset, per_page).await?;

        let venue_ids: Vec<u64> = entities.iter().filter_map(|e| e.venue_id).collect();
        let mut venues: HashMap<u64, VenueEntity> = self
            .venue_repo
            .find_by_ids(&venue_ids)
            .await?
            .into_iter()
            .map(|v| (v.id, v))
            .collect();

        let conferences = entities
            .into_iter()
            .map(|e| {
                let venue = e.venue_id.and_then(|id| venues.remove(&id));
                Conference::from(e).with_venue(venue)
            })
            .collect();

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

        let conference = Conference::from(conference).with_venue(venue);

        Ok(conference)
    }

    async fn create(&self, dto: CreateConferenceRequest) -> Result<Conference, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Domain(DomainError::Database(e.to_string())))?;

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
            group_discount_id: None,
            published: 0,
            created_at: now,
            updated_at: now,
        };

        let conference_entity = self
            .conference_repo
            .create_in_tx(&mut tx, conference_entity)
            .await?;

        if conference_entity.start_date.is_some() {
            let price_tiers = generate_price_tiers(dto.start_date.unwrap().into());
            let price_tiers = price_tiers
                .into_iter()
                .map(|e| {
                    let now = Utc::now();

                    PriceTierEntity {
                        id: 0,
                        conference_id: conference_entity.id,
                        price: e.price,
                        deadline: e.deadline,
                        created_at: now,
                        updated_at: now,
                    }
                })
                .collect::<Vec<PriceTierEntity>>();

            self.price_tier_repo
                .create_many_in_tx(&mut tx, price_tiers)
                .await
                .map_err(|e| AppError::Domain(DomainError::Database(e.to_string())))?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::Domain(DomainError::Database(e.to_string())))?;

        unimplemented!(
            "If the created conference has a venue, at the moment, it is not returning the venue as well. When you encounter this error, fix it"
        );

        Ok(Conference::from(conference_entity))
    }

    async fn generate_price_tiers(&self, id: u64) -> Result<Vec<PriceTier>, AppError> {
        // Update existing stored price tiers
        let conference = self.conference_repo.find_by_id(id).await?;

        let start_date = conference.start_date.ok_or_else(|| {
            AppError::Domain(DomainError::InvalidTransition(
                "Can't generate the price tiers without a start date".to_string(),
            ))
        })?;

        let original_price_tiers = self
            .price_tier_repo
            .find_by_conference_id(conference.id)
            .await?;

        let generated_price_tiers = generate_price_tiers(start_date.date());

        let price_tiers: Vec<PriceTierEntity> = original_price_tiers
            .into_iter()
            .zip(generated_price_tiers.into_iter())
            .map(|(mut entity, generated)| {
                entity.price = generated.price;
                entity.deadline = generated.deadline;
                entity.updated_at = Utc::now();
                entity
            })
            .collect();

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Domain(DomainError::Database(e.to_string())))?;

        let price_tiers = self
            .price_tier_repo
            .update_many(&mut tx, price_tiers)
            .await?;

        tx.commit()
            .await
            .map_err(|e| AppError::Domain(DomainError::Database(e.to_string())))?;

        Ok(price_tiers.into_iter().map(PriceTier::from).collect())
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
            group_discount_id: dto
                .group_discount_id
                .or(conference_entity.group_discount_id),
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

        Ok(Conference::from(conference_entity).with_venue(venue_entity))
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

        Ok(Conference::from(entity).with_venue(venue))
    }
}
