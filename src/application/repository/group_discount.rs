use crate::application::{entity::GroupDiscountEntity, repository::Repository};

#[async_trait::async_trait]
pub trait GroupDiscountRepository: Repository<GroupDiscountEntity> {}
