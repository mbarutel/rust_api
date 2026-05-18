use std::sync::Arc;

use chrono::Utc;

use crate::{
    application::{
        dto::{CreateGroupDiscountRequest, UpdateGroupDiscountRequest},
        entity::GroupDiscountEntity,
        error::AppError,
        repository::GroupDiscountRepository,
        service::group_discount::GroupDiscountService,
    },
    domain::models::GroupDiscount,
};

pub struct GroupDiscountServiceImpl {
    group_discount_repo: Arc<dyn GroupDiscountRepository>,
}

impl GroupDiscountServiceImpl {
    pub fn new(group_discount_repo: Arc<dyn GroupDiscountRepository>) -> Self {
        Self {
            group_discount_repo,
        }
    }
}

#[async_trait::async_trait]
impl GroupDiscountService for GroupDiscountServiceImpl {
    async fn list(&self, page: u32, per_page: u32) -> Result<(Vec<GroupDiscount>, u64), AppError> {
        let offset = (page - 1) * per_page;
        let total = self.group_discount_repo.count().await?;
        let entities = self.group_discount_repo.find_all(offset, per_page).await?;
        let group_discounts = entities.into_iter().map(GroupDiscount::from).collect();

        Ok((group_discounts, total))
    }

    async fn find_by_id(&self, id: u64) -> Result<GroupDiscount, AppError> {
        let entity = self.group_discount_repo.find_by_id(id).await?;

        Ok(GroupDiscount::from(entity))
    }

    async fn create(&self, dto: CreateGroupDiscountRequest) -> Result<GroupDiscount, AppError> {
        let now = Utc::now();
        let entity = GroupDiscountEntity {
            id: 0,
            name: dto.name,
            code: dto.code,
            min_quantity: dto.min_quantity,
            free_quantity: dto.free_quantity,
            active: 0,
            valid_until: dto.valid_until,
            created_at: now,
            updated_at: now,
        };

        Ok(GroupDiscount::from(
            self.group_discount_repo.create(entity).await?,
        ))
    }

    async fn update(
        &self,
        id: u64,
        dto: UpdateGroupDiscountRequest,
    ) -> Result<GroupDiscount, AppError> {
        let entity = self.group_discount_repo.find_by_id(id).await?;

        let updated_entity = GroupDiscountEntity {
            id,
            name: dto.name.unwrap_or(entity.name),
            min_quantity: dto.min_quantity.unwrap_or(entity.min_quantity),
            free_quantity: dto.min_quantity.unwrap_or(entity.free_quantity),
            valid_until: dto.valid_until.or(entity.valid_until),
            updated_at: Utc::now(),
            ..entity
        };

        Ok(GroupDiscount::from(
            self.group_discount_repo.update(updated_entity).await?,
        ))
    }

    async fn delete(&self, id: u64) -> Result<(), AppError> {
        Ok(self.group_discount_repo.delete(id).await?)
    }

    async fn toggle_activate(&self, id: u64) -> Result<GroupDiscount, AppError> {
        let entity = self.group_discount_repo.find_by_id(id).await?;
        let updated_entity = GroupDiscountEntity {
            active: !entity.active as i8,
            updated_at: Utc::now(),
            ..entity
        };
        Ok(GroupDiscount::from(
            self.group_discount_repo.update(updated_entity).await?,
        ))
    }
}
