use crate::{
    application::{
        dto::masterclass_dto::{CreateMasterclassRequest, UpdateMasterclassRequest},
        error::AppError,
    },
    domain::models::masterclass::{Masterclass, MasterclassInstructor},
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MasterclassService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Masterclass>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<Masterclass, AppError>;
    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<Masterclass>, AppError>;
    async fn create(&self, dto: CreateMasterclassRequest) -> Result<Masterclass, AppError>;
    async fn update(
        &self,
        id: u64,
        dto: UpdateMasterclassRequest,
    ) -> Result<Masterclass, AppError>;
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
}
