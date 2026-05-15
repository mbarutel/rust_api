use crate::{
    application::{
        entity::GroupDiscountEntity,
        repository::{GroupDiscountRepository, Repository},
    },
    domain::error::DomainError,
};

use super::macros::{map_db_err, map_find_err};

pub struct GroupDiscountRepositoryImpl {
    pub pool: sqlx::MySqlPool,
}

impl GroupDiscountRepositoryImpl {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Repository<GroupDiscountEntity> for GroupDiscountRepositoryImpl {
    async fn find_all(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<GroupDiscountEntity>, DomainError> {
        sqlx::query_as!(
            GroupDiscountEntity,
            "
                SELECT
                    id,
                    name,
                    min_quantity,
                    free_quantity,
                    active,
                    valid_until,
                    created_at,
                    updated_at
                 FROM
                     group_discounts
                 LIMIT
                     ?
                 OFFSET
                     ?
            ",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn find_by_id(&self, id: u64) -> Result<GroupDiscountEntity, DomainError> {
        sqlx::query_as!(
            GroupDiscountEntity,
            "
                SELECT
                    id,
                    name,
                    min_quantity,
                    free_quantity,
                    active,
                    valid_until,
                    created_at,
                    updated_at
                FROM
                    group_discounts
                WHERE
                    id = ?
            ",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn create(
        &self,
        entity: GroupDiscountEntity,
    ) -> Result<GroupDiscountEntity, DomainError> {
        let id = sqlx::query!(
            "INSERT INTO
                group_discounts (
                    name,
                    min_quantity,
                    free_quantity,
                    active,
                    valid_until,
                    created_at,
                    updated_at
                )
                VALUES (
                    ?, ?, ?, ?, ?, ?, ?
                )",
            entity.name,
            entity.min_quantity,
            entity.free_quantity,
            entity.active,
            entity.valid_until,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?
        .last_insert_id();

        Ok(GroupDiscountEntity { id, ..entity })
    }

    async fn update(
        &self,
        entity: GroupDiscountEntity,
    ) -> Result<GroupDiscountEntity, DomainError> {
        sqlx::query!(
            "UPDATE
                group_discounts
            SET
                name = ?,
                min_quantity = ?,
                free_quantity = ?,
                active = ?,
                valid_until = ?,
                updated_at = ?
            WHERE
                id = ?",
            entity.name,
            entity.min_quantity,
            entity.free_quantity,
            entity.active,
            entity.valid_until,
            entity.updated_at,
            entity.id
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(entity)
    }

    async fn delete(&self, id: u64) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM group_discounts WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(())
    }

    async fn count(&self) -> Result<u64, DomainError> {
        let count = sqlx::query_scalar("SELECT COUNT(*) FROM group_discountsO")
            .fetch_one(&self.pool)
            .await
            .map_err(map_db_err)?;

        Ok(count)
    }
}

#[async_trait::async_trait]
impl GroupDiscountRepository for GroupDiscountRepositoryImpl {}
