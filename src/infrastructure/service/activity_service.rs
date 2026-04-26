use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::activity_dto::{CreateActivityRequest, UpdateActivityRequest},
        entity::activity_entity::ActivityEntity,
        error::AppError,
        repository::{
            activity_booking_repository::ActivityBookingRepository,
            activity_repository::ActivityRepository,
        },
        service::activity_service::ActivityService,
    },
    domain::models::{activity::Activity, activity_booking::ActivityBooking},
};

pub struct ActivityServiceImpl {
    activity_repo: Arc<dyn ActivityRepository>,
    booking_repo: Arc<dyn ActivityBookingRepository>,
}

impl ActivityServiceImpl {
    pub fn new(
        activity_repo: Arc<dyn ActivityRepository>,
        booking_repo: Arc<dyn ActivityBookingRepository>,
    ) -> Self {
        Self {
            activity_repo,
            booking_repo,
        }
    }
}

#[async_trait::async_trait]
impl ActivityService for ActivityServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Activity>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.activity_repo.count().await?;
        let activities = self
            .activity_repo
            .find_all(offset, per_page)
            .await?
            .into_iter()
            .map(Activity::from)
            .collect();
        Ok((activities, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<Activity, AppError> {
        Ok(Activity::from(self.activity_repo.find_by_id(id).await?))
    }

    async fn find_by_conference(&self, conference_id: u64) -> Result<Vec<Activity>, AppError> {
        Ok(self
            .activity_repo
            .find_by_conference(conference_id)
            .await?
            .into_iter()
            .map(Activity::from)
            .collect())
    }

    async fn create(&self, dto: CreateActivityRequest) -> Result<Activity, AppError> {
        let entity = ActivityEntity {
            id: 0,
            conference_id: dto.conference_id,
            name: dto.name,
            description: dto.description,
            start_at: dto.start_at,
            end_at: dto.end_at,
            venue_id: dto.venue_id,
            provider_url: dto.provider_url,
            capacity: dto.capacity,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(Activity::from(self.activity_repo.create(entity).await?))
    }

    async fn update(&self, id: u64, dto: UpdateActivityRequest) -> Result<Activity, AppError> {
        let entity = self.activity_repo.find_by_id(id).await?;
        let entity = ActivityEntity {
            name: dto.name.unwrap_or(entity.name),
            description: dto.description.or(entity.description),
            start_at: dto.start_at.unwrap_or(entity.start_at),
            end_at: dto.end_at.unwrap_or(entity.end_at),
            venue_id: dto.venue_id.or(entity.venue_id),
            provider_url: dto.provider_url.or(entity.provider_url),
            capacity: dto.capacity.or(entity.capacity),
            updated_at: Utc::now(),
            ..entity
        };
        Ok(Activity::from(self.activity_repo.update(entity).await?))
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.activity_repo.delete(id).await?)
    }

    async fn book(&self, activity_id: u64, participant_id: u64) -> Result<(), AppError> {
        Ok(self.booking_repo.book(activity_id, participant_id).await?)
    }

    async fn confirm_booking(
        &self,
        activity_id: u64,
        participant_id: u64,
    ) -> Result<(), AppError> {
        Ok(self
            .booking_repo
            .confirm(activity_id, participant_id)
            .await?)
    }

    async fn cancel_booking(
        &self,
        activity_id: u64,
        participant_id: u64,
    ) -> Result<(), AppError> {
        Ok(self
            .booking_repo
            .cancel(activity_id, participant_id)
            .await?)
    }

    async fn list_bookings_by_activity(
        &self,
        activity_id: u64,
    ) -> Result<Vec<ActivityBooking>, AppError> {
        Ok(self
            .booking_repo
            .find_by_activity(activity_id)
            .await?
            .into_iter()
            .map(ActivityBooking::from)
            .collect())
    }

    async fn list_bookings_by_participant(
        &self,
        participant_id: u64,
    ) -> Result<Vec<ActivityBooking>, AppError> {
        Ok(self
            .booking_repo
            .find_by_participant(participant_id)
            .await?
            .into_iter()
            .map(ActivityBooking::from)
            .collect())
    }
}
