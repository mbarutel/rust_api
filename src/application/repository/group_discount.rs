use crate::application::{entity::GroupDiscountEntity, repository::Repository};

pub trait GroupDiscountRepository: Repository<GroupDiscountEntity> {}
