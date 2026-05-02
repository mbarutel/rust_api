use crate::{
    application::{entity::price_tier_entity::PriceTierEntity, repository::Repository},
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

#[async_trait::async_trait]
pub trait PriceTierRepository: Repository<PriceTierEntity> {
    async fn create_many_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entities: Vec<PriceTierEntity>,
    ) -> Result<Vec<PriceTierEntity>, DomainError>;
}

db_repository!(DbPriceTierRepository);

#[async_trait::async_trait]
impl Repository<PriceTierEntity> for DbPriceTierRepository {
    async fn find_by_id(&self, id: u64) -> Result<PriceTierEntity, DomainError> {
        sqlx::query_as!(
            PriceTierEntity,
            "SELECT
                id,
                conference_id,
                price,
                deadline,
                created_at,
                updated_at
            FROM price_tiers
            WHERE id = ?",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<PriceTierEntity>, DomainError> {
        sqlx::query_as!(
            PriceTierEntity,
            "SELECT
                id,
                conference_id,
                price,
                deadline,
                created_at,
                updated_at
            FROM price_tiers
            LIMIT ? OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, entity: PriceTierEntity) -> Result<PriceTierEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO price_tiers (
                conference_id,
                price,
                deadline,
                created_at,
                updated_at
            ) VALUES (?, ?, ?, ?, ?)",
            entity.conference_id,
            entity.price,
            entity.deadline,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(entity)
    }

    async fn update(&self, entity: PriceTierEntity) -> Result<PriceTierEntity, DomainError> {
        sqlx::query!(
            "UPDATE price_tiers SET
                conference_id = ?,
                price = ?,
                deadline = ?,
                updated_at = ?
            WHERE id = ?",
            entity.conference_id,
            entity.price,
            entity.deadline,
            entity.updated_at,
            entity.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(entity)
    }

    impl_count!("price_tiers");
    impl_delete!("price_tiers");
}

#[async_trait::async_trait]
impl PriceTierRepository for DbPriceTierRepository {
    async fn create_many_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entities: Vec<PriceTierEntity>,
    ) -> Result<Vec<PriceTierEntity>, DomainError> {
        let mut inserted = Vec::with_capacity(entities.len());

        for entity in entities {
            let result = sqlx::query!(
                "INSERT INTO price_tiers (
                    conference_id,
                    price,
                    deadline,
                    created_at,
                    updated_at
                ) VALUES (?, ?, ?, ?, ?)",
                entity.conference_id,
                entity.price,
                entity.deadline,
                entity.created_at,
                entity.updated_at,
            )
            .execute(&mut **tx)
            .await
            .map_err(map_db_err)?;

            inserted.push(PriceTierEntity {
                id: result.last_insert_id(),
                ..entity
            });
        }

        Ok(inserted)
    }
}
