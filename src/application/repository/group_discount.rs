use crate::{
    application::{entity::GroupDiscountEntity, repository::Repository},
    domain::error::DomainError,
};

#[async_trait::async_trait]
pub trait GroupDiscountRepository: Repository<GroupDiscountEntity> {
    async fn find_by_code(&self, code: &str) -> Result<Option<GroupDiscountEntity>, DomainError>;
}
