use crate::{
    application::{
        dto::masterclass::{CreateMasterclassRequest, UpdateMasterclassRequest},
        error::AppError,
    },
    domain::models::{
        masterclass::{Masterclass, MasterclassInstructor},
        masterclass_booking::MasterclassBooking,
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MasterclassService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Masterclass>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<Masterclass, AppError>;
    async fn find_by_conference(&self, conference_id: u64) -> Result<Vec<Masterclass>, AppError>;
    async fn create(&self, dto: CreateMasterclassRequest) -> Result<Masterclass, AppError>;
    async fn update(&self, id: u64, dto: UpdateMasterclassRequest)
    -> Result<Masterclass, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
    async fn add_instructor(
        &self,
        masterclass_id: u64,
        participant_id: u64,
        is_lead: bool,
    ) -> Result<(), AppError>;
    async fn remove_instructor(
        &self,
        masterclass_id: u64,
        participant_id: u64,
    ) -> Result<(), AppError>;
    async fn list_instructors(
        &self,
        masterclass_id: u64,
    ) -> Result<Vec<MasterclassInstructor>, AppError>;
    async fn book(&self, masterclass_id: u64, participant_id: u64) -> Result<(), AppError>;
    async fn confirm_booking(
        &self,
        masterclass_id: u64,
        participant_id: u64,
    ) -> Result<(), AppError>;
    async fn cancel_booking(
        &self,
        masterclass_id: u64,
        participant_id: u64,
    ) -> Result<(), AppError>;
    async fn list_bookings_by_masterclass(
        &self,
        masterclass_id: u64,
    ) -> Result<Vec<MasterclassBooking>, AppError>;
    async fn list_bookings_by_participant(
        &self,
        participant_id: u64,
    ) -> Result<Vec<MasterclassBooking>, AppError>;
}
