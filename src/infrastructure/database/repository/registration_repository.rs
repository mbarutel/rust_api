use crate::{
    application::{
        entity::registration_entity::RegistrationEntity,
        repository::{Repository, registration_repository::RegistrationRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
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

    async fn create(
        &self,
        registration: RegistrationEntity,
    ) -> Result<RegistrationEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO registration (
                conference_id,
                status,
                cost,
                discount_code,
                discount_amount,
                amount_paid,
                created_by_id,
                notes_internal,
                created_at,
                updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            registration.conference_id,
            registration.status,
            registration.cost,
            registration.discount_code,
            registration.discount_amount,
            registration.amount_paid,
            registration.created_by_id,
            registration.notes_internal,
            registration.created_at,
            registration.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(registration)
    }

    async fn update(
        &self,
        registration: RegistrationEntity,
    ) -> Result<RegistrationEntity, DomainError> {
        sqlx::query!(
            "UPDATE registration SET
                status = ?,
                cost = ?,
                discount_code = ?,
                discount_amount = ?,
                amount_paid = ?,
                notes_internal = ?,
                updated_at = ?
            WHERE id = ?",
            registration.status,
            registration.cost,
            registration.discount_code,
            registration.discount_amount,
            registration.amount_paid,
            registration.notes_internal,
            registration.updated_at,
            registration.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(registration)
    }

    impl_count!("registration");
    impl_delete!("registration");
}

#[async_trait::async_trait]
impl RegistrationRepository for DbRegistrationRepository {
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
                notes_internal,
                created_at,
                updated_at
            FROM registration
            WHERE conference_id = ?",
            conference_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }
}
