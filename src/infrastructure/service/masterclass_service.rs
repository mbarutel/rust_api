use chrono::Utc;
use std::sync::Arc;

use crate::{
    application::{
        dto::masterclass_dto::{CreateMasterclassRequest, UpdateMasterclassRequest},
        entity::masterclass_entity::MasterclassEntity,
        error::AppError,
        repository::masterclass_booking_repository::MasterclassBookingRepository,
        repository::masterclass_repository::{
            MasterclassInstructorRepository, MasterclassRepository,
        },
        service::masterclass_service::MasterclassService,
    },
    domain::models::{
        masterclass::{Masterclass, MasterclassInstructor},
        masterclass_booking::MasterclassBooking,
    },
};

pub struct MasterclassServiceImpl {
    masterclass_repo: Arc<dyn MasterclassRepository>,
    instructor_repo: Arc<dyn MasterclassInstructorRepository>,
    booking_repo: Arc<dyn MasterclassBookingRepository>,
}

impl MasterclassServiceImpl {
    pub fn new(
        masterclass_repo: Arc<dyn MasterclassRepository>,
        instructor_repo: Arc<dyn MasterclassInstructorRepository>,
        booking_repo: Arc<dyn MasterclassBookingRepository>,
    ) -> Self {
        Self {
            masterclass_repo,
            instructor_repo,
            booking_repo,
        }
    }
}

#[async_trait::async_trait]
impl MasterclassService for MasterclassServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Masterclass>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.masterclass_repo.count().await?;
        let masterclasses = self
            .masterclass_repo
            .find_all(offset, per_page)
            .await?
            .into_iter()
            .map(Masterclass::from)
            .collect();
        Ok((masterclasses, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<Masterclass, AppError> {
        Ok(Masterclass::from(
            self.masterclass_repo.find_by_id(id).await?,
        ))
    }

    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<Masterclass>, AppError> {
        Ok(self
            .masterclass_repo
            .find_by_conference(conference_id)
            .await?
            .into_iter()
            .map(Masterclass::from)
            .collect())
    }

    async fn create(&self, dto: CreateMasterclassRequest) -> Result<Masterclass, AppError> {
        let entity = MasterclassEntity {
            id: 0,
            conference_id: dto.conference_id,
            name: dto.name,
            description: dto.description,
            start_at: dto.start_at,
            end_at: dto.end_at,
            venue_id: dto.venue_id,
            capacity: dto.capacity,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(Masterclass::from(
            self.masterclass_repo.create(entity).await?,
        ))
    }

    async fn update(
        &self,
        id: u64,
        dto: UpdateMasterclassRequest,
    ) -> Result<Masterclass, AppError> {
        let entity = self.masterclass_repo.find_by_id(id).await?;
        let entity = MasterclassEntity {
            name: dto.name.unwrap_or(entity.name),
            description: dto.description.or(entity.description),
            start_at: dto.start_at.unwrap_or(entity.start_at),
            end_at: dto.end_at.unwrap_or(entity.end_at),
            venue_id: dto.venue_id.or(entity.venue_id),
            capacity: dto.capacity.or(entity.capacity),
            updated_at: Utc::now(),
            ..entity
        };
        Ok(Masterclass::from(
            self.masterclass_repo.update(entity).await?,
        ))
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.masterclass_repo.delete(id).await?)
    }

    async fn add_instructor(
        &self,
        masterclass_id: u64,
        participant_id: u64,
        is_lead: bool,
    ) -> Result<(), AppError> {
        Ok(self
            .instructor_repo
            .add(masterclass_id, participant_id, is_lead)
            .await?)
    }

    async fn remove_instructor(
        &self,
        masterclass_id: u64,
        participant_id: u64,
    ) -> Result<(), AppError> {
        Ok(self
            .instructor_repo
            .remove(masterclass_id, participant_id)
            .await?)
    }

    async fn list_instructors(
        &self,
        masterclass_id: u64,
    ) -> Result<Vec<MasterclassInstructor>, AppError> {
        Ok(self
            .instructor_repo
            .find_by_masterclass(masterclass_id)
            .await?
            .into_iter()
            .map(MasterclassInstructor::from)
            .collect())
    }

    async fn book(&self, masterclass_id: u64, participant_id: u64) -> Result<(), AppError> {
        Ok(self.booking_repo.book(masterclass_id, participant_id).await?)
    }

    async fn confirm_booking(
        &self,
        masterclass_id: u64,
        participant_id: u64,
    ) -> Result<(), AppError> {
        Ok(self
            .booking_repo
            .confirm(masterclass_id, participant_id)
            .await?)
    }

    async fn cancel_booking(
        &self,
        masterclass_id: u64,
        participant_id: u64,
    ) -> Result<(), AppError> {
        Ok(self
            .booking_repo
            .cancel(masterclass_id, participant_id)
            .await?)
    }

    async fn list_bookings_by_masterclass(
        &self,
        masterclass_id: u64,
    ) -> Result<Vec<MasterclassBooking>, AppError> {
        Ok(self
            .booking_repo
            .find_by_masterclass(masterclass_id)
            .await?
            .into_iter()
            .map(MasterclassBooking::from)
            .collect())
    }

    async fn list_bookings_by_participant(
        &self,
        participant_id: u64,
    ) -> Result<Vec<MasterclassBooking>, AppError> {
        Ok(self
            .booking_repo
            .find_by_participant(participant_id)
            .await?
            .into_iter()
            .map(MasterclassBooking::from)
            .collect())
    }
}
