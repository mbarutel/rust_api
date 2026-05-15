use crate::{
    application::{
        dto::group_discount::{CreateGroupDiscountRequest, UpdateGroupDiscountRequest},
        error::AppError,
    },
    domain::models::GroupDiscount,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait GroupDiscountService: Send + Sync {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<GroupDiscount>, u64), AppError>;
    async fn find_by_id(&self, id: u64) -> Result<GroupDiscount, AppError>;
    async fn create(&self, dto: CreateGroupDiscountRequest) -> Result<GroupDiscount, AppError>;
    async fn update(
        &self,
        id: u64,
        dto: UpdateGroupDiscountRequest,
    ) -> Result<GroupDiscount, AppError>;
    async fn delete(&self, id: u64) -> Result<(), AppError>;
    async fn toggle_activate(&self, id: u64) -> Result<GroupDiscount, AppError>;
}
