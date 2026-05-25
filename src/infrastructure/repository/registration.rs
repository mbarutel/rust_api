use crate::{
    application::{
        entity::registration::RegistrationEntity,
        repository::{Repository, registration::RegistrationRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbRegistrationRepository);

#[async_trait::async_trait]
impl Repository<RegistrationEntity> for DbRegistrationRepository {
    async fn find_by_id(&self, id: u64) -> Result<RegistrationEntity, DomainError> {
        sqlx::query_as!(
            RegistrationEntity,
            "SELECT
                id,
                conference_id,
                status,
                cost,
                discount_code,
                discount_amount,
                amount_paid,
                created_by_id,
                referrer,
                notes_internal,
                created_at,
                updated_at
            FROM registration
            WHERE id = ?",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn find_all(
        &self,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<RegistrationEntity>, DomainError> {
        sqlx::query_as!(
            RegistrationEntity,
            "SELECT
                id,
                conference_id,
                status,
                cost,
                discount_code,
                discount_amount,
                amount_paid,
                created_by_id,
                referrer,
                notes_internal,
                created_at,
                updated_at
            FROM registration
            LIMIT ? OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, entity: RegistrationEntity) -> Result<RegistrationEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO registration (
                conference_id,
                status,
                cost,
                discount_code,
                discount_amount,
                amount_paid,
                created_by_id,
                referrer,
                notes_internal,
                created_at,
                updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            entity.conference_id,
            entity.status,
            entity.cost,
            entity.discount_code,
            entity.discount_amount,
            entity.amount_paid,
            entity.created_by_id,
            entity.referrer,
            entity.notes_internal,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(entity)
    }

    async fn update(&self, entity: RegistrationEntity) -> Result<RegistrationEntity, DomainError> {
        sqlx::query!(
            "UPDATE registration SET
                status = ?,
                cost = ?,
                discount_code = ?,
                discount_amount = ?,
                amount_paid = ?,
                referrer = ?,
                notes_internal = ?,
                updated_at = ?
            WHERE id = ?",
            entity.status,
            entity.cost,
            entity.discount_code,
            entity.discount_amount,
            entity.amount_paid,
            entity.referrer,
            entity.notes_internal,
            entity.updated_at,
            entity.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(entity)
    }

    impl_count!("registration");
    impl_delete!("registration");
}

#[async_trait::async_trait]
impl RegistrationRepository for DbRegistrationRepository {
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: RegistrationEntity,
    ) -> Result<RegistrationEntity, DomainError> {
        let result = sqlx::query!(
            "INSERT INTO registration (
                conference_id,
                status,
                cost,
                discount_code,
                discount_amount,
                amount_paid,
                created_by_id,
                referrer,
                notes_internal,
                created_at,
                updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            entity.conference_id,
            entity.status,
            entity.cost,
            entity.discount_code,
            entity.discount_amount,
            entity.amount_paid,
            entity.created_by_id,
            entity.referrer,
            entity.notes_internal,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&mut **tx)
        .await
        .map_err(map_db_err)?;

        Ok(RegistrationEntity {
            id: result.last_insert_id(),
            ..entity
        })
    }

    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<RegistrationEntity>, DomainError> {
        sqlx::query_as!(
            RegistrationEntity,
            "SELECT
                id,
                conference_id,
                status,
                cost,
                discount_code,
                discount_amount,
                amount_paid,
                created_by_id,
                referrer,
                notes_internal,
                created_at,
                updated_at
            FROM
                registration
            WHERE
                conference_id = ?",
            conference_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }
}
