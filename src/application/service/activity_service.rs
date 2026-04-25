use crate::{
    application::{
        dto::activity_dto::{CreateActivityRequest, UpdateActivityRequest},
        error::AppError,
    },
    domain::models::activity::Activity,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ActivityService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<Activity>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<Activity, AppError>;
    async fn find_by_conference(&self, conference_id: u64) -> Result<Vec<Activity>, AppError>;
    async fn create(&self, dto: CreateActivityRequest) -> Result<Activity, AppError>;
    async fn update(&self, id: u64, dto: UpdateActivityRequest) -> Result<Activity, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
}
