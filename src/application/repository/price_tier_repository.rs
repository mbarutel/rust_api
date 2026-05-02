// use crate::{
//     application::{entity::price_tier_entity::PriceTierEntity, repository::Repository},
//     domain::error::DomainError,
// };

// #[async_trait::async_trait]
// pub trait PriceTierRepository: Repository<PriceTierEntity> {
//     async fn create_many_in_tx(
//         &self,
//         tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
//         entities: Vec<PriceTierEntity>,
//     ) -> Result<Vec<PriceTierEntity>, DomainError>;
// }
