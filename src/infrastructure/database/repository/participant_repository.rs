use crate::{
    application::{
        entity::participant_entity::ParticipantEntity,
        repository::{Repository, participant_repository::ParticipantRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbParticipantRepository);

#[async_trait::async_trait]
impl Repository<ParticipantEntity> for DbParticipantRepository {
    async fn find_by_id(&self, id: u64) -> Result<ParticipantEntity, DomainError> {
        sqlx::query_as!(
            ParticipantEntity,
            "SELECT
                id,
                registration_id,
                client_id,
                participant_role,
                dietary_requirements,
                accessibility_needs,
                created_at,
                updated_at
            FROM participants
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
    ) -> Result<Vec<ParticipantEntity>, DomainError> {
        sqlx::query_as!(
            ParticipantEntity,
            "SELECT
                id,
                registration_id,
                client_id,
                participant_role,
                dietary_requirements,
                accessibility_needs,
                created_at,
                updated_at
            FROM participants
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
        participant: ParticipantEntity,
    ) -> Result<ParticipantEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO participants (
                registration_id,
                client_id,
                participant_role,
                dietary_requirements,
                accessibility_needs,
                created_at,
                updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            participant.registration_id,
            participant.client_id,
            participant.participant_role,
            participant.dietary_requirements,
            participant.accessibility_needs,
            participant.created_at,
            participant.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(participant)
    }

    async fn update(
        &self,
        participant: ParticipantEntity,
    ) -> Result<ParticipantEntity, DomainError> {
        sqlx::query!(
            "UPDATE participants SET
                participant_role = ?,
                dietary_requirements = ?,
                accessibility_needs = ?,
                updated_at = ?
            WHERE id = ?",
            participant.participant_role,
            participant.dietary_requirements,
            participant.accessibility_needs,
            participant.updated_at,
            participant.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(participant)
    }

    impl_count!("participants");
    impl_delete!("participants");
}

#[async_trait::async_trait]
impl ParticipantRepository for DbParticipantRepository {
    async fn find_by_registration(
        &self,
        registration_id: u64,
    ) -> Result<Vec<ParticipantEntity>, DomainError> {
        sqlx::query_as!(
            ParticipantEntity,
            "SELECT
                id,
                registration_id,
                client_id,
                participant_role,
                dietary_requirements,
                accessibility_needs,
                created_at,
                updated_at
            FROM participants
            WHERE registration_id = ?",
            registration_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }
}
