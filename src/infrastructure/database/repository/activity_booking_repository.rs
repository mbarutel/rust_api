use chrono::Utc;

use crate::{
    application::{
        entity::activity_booking_entity::ActivityBookingEntity,
        repository::activity_booking_repository::ActivityBookingRepository,
    },
    db_repository,
    domain::error::DomainError,
    infrastructure::database::repository::macros::map_db_err,
};

db_repository!(DbActivityBookingRepository);

#[async_trait::async_trait]
impl ActivityBookingRepository for DbActivityBookingRepository {
    async fn book(&self, activity_id: u64, participant_id: u64) -> Result<(), DomainError> {
        let now = Utc::now();
        sqlx::query!(
            "INSERT INTO activity_bookings
                (activity_id, participant_id, status, created_at, updated_at)
             VALUES (?, ?, 'reserved', ?, ?)",
            activity_id,
            participant_id,
            now,
            now,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(())
    }

    async fn confirm(&self, activity_id: u64, participant_id: u64) -> Result<(), DomainError> {
        sqlx::query!(
            "UPDATE activity_bookings SET status = 'confirmed', updated_at = ?
             WHERE activity_id = ? AND participant_id = ?",
            Utc::now(),
            activity_id,
            participant_id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(())
    }

    async fn cancel(&self, activity_id: u64, participant_id: u64) -> Result<(), DomainError> {
        sqlx::query!(
            "UPDATE activity_bookings SET status = 'cancelled', updated_at = ?
             WHERE activity_id = ? AND participant_id = ?",
            Utc::now(),
            activity_id,
            participant_id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(())
    }

    async fn find_by_activity(
        &self,
        activity_id: u64,
    ) -> Result<Vec<ActivityBookingEntity>, DomainError> {
        sqlx::query_as!(
            ActivityBookingEntity,
            "SELECT id, activity_id, participant_id, status, created_at, updated_at
             FROM activity_bookings WHERE activity_id = ?",
            activity_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn find_by_participant(
        &self,
        participant_id: u64,
    ) -> Result<Vec<ActivityBookingEntity>, DomainError> {
        sqlx::query_as!(
            ActivityBookingEntity,
            "SELECT id, activity_id, participant_id, status, created_at, updated_at
             FROM activity_bookings WHERE participant_id = ?",
            participant_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }
}
