use chrono::Utc;

use crate::{
    application::{
        entity::masterclass_booking_entity::MasterclassBookingEntity,
        repository::masterclass_booking_repository::MasterclassBookingRepository,
    },
    db_repository,
    domain::error::DomainError,
    infrastructure::database::repository::macros::map_db_err,
};

db_repository!(DbMasterclassBookingRepository);

#[async_trait::async_trait]
impl MasterclassBookingRepository for DbMasterclassBookingRepository {
    async fn book(&self, masterclass_id: u64, participant_id: u64) -> Result<(), DomainError> {
        let now = Utc::now();
        sqlx::query!(
            "INSERT INTO masterclass_bookings
                (masterclass_id, participant_id, status, created_at, updated_at)
             VALUES (?, ?, 'reserved', ?, ?)",
            masterclass_id,
            participant_id,
            now,
            now,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(())
    }

    async fn confirm(
        &self,
        masterclass_id: u64,
        participant_id: u64,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            "UPDATE masterclass_bookings SET status = 'confirmed', updated_at = ?
             WHERE masterclass_id = ? AND participant_id = ?",
            Utc::now(),
            masterclass_id,
            participant_id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(())
    }

    async fn cancel(
        &self,
        masterclass_id: u64,
        participant_id: u64,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            "UPDATE masterclass_bookings SET status = 'cancelled', updated_at = ?
             WHERE masterclass_id = ? AND participant_id = ?",
            Utc::now(),
            masterclass_id,
            participant_id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(())
    }

    async fn find_by_masterclass(
        &self,
        masterclass_id: u64,
    ) -> Result<Vec<MasterclassBookingEntity>, DomainError> {
        sqlx::query_as!(
            MasterclassBookingEntity,
            "SELECT id, masterclass_id, participant_id, status, created_at, updated_at
             FROM masterclass_bookings WHERE masterclass_id = ?",
            masterclass_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn find_by_participant(
        &self,
        participant_id: u64,
    ) -> Result<Vec<MasterclassBookingEntity>, DomainError> {
        sqlx::query_as!(
            MasterclassBookingEntity,
            "SELECT id, masterclass_id, participant_id, status, created_at, updated_at
             FROM masterclass_bookings WHERE participant_id = ?",
            participant_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }
}
