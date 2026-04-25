use chrono::Utc;

use crate::{
    application::{
        entity::masterclass_entity::{MasterclassEntity, MasterclassInstructorEntity},
        repository::{
            Repository,
            masterclass_repository::{MasterclassInstructorRepository, MasterclassRepository},
        },
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbMasterclassRepository);
db_repository!(DbMasterclassInstructorRepository);

#[async_trait::async_trait]
impl Repository<MasterclassEntity> for DbMasterclassRepository {
    async fn find_by_id(&self, id: u64) -> Result<MasterclassEntity, DomainError> {
        sqlx::query_as!(
            MasterclassEntity,
            "SELECT id, conference_id, name, description, start_at, end_at,
             venue_id, capacity, created_at, updated_at
             FROM masterclasses WHERE id = ?",
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
    ) -> Result<Vec<MasterclassEntity>, DomainError> {
        sqlx::query_as!(
            MasterclassEntity,
            "SELECT id, conference_id, name, description, start_at, end_at,
             venue_id, capacity, created_at, updated_at
             FROM masterclasses LIMIT ? OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, entity: MasterclassEntity) -> Result<MasterclassEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO masterclasses (
                conference_id, name, description, start_at, end_at,
                venue_id, capacity, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            entity.conference_id,
            entity.name,
            entity.description,
            entity.start_at,
            entity.end_at,
            entity.venue_id,
            entity.capacity,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    async fn update(&self, entity: MasterclassEntity) -> Result<MasterclassEntity, DomainError> {
        sqlx::query!(
            "UPDATE masterclasses SET
                name = ?, description = ?, start_at = ?, end_at = ?,
                venue_id = ?, capacity = ?, updated_at = ?
             WHERE id = ?",
            entity.name,
            entity.description,
            entity.start_at,
            entity.end_at,
            entity.venue_id,
            entity.capacity,
            entity.updated_at,
            entity.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    impl_count!("masterclasses");
    impl_delete!("masterclasses");
}

#[async_trait::async_trait]
impl MasterclassRepository for DbMasterclassRepository {
    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<MasterclassEntity>, DomainError> {
        sqlx::query_as!(
            MasterclassEntity,
            "SELECT id, conference_id, name, description, start_at, end_at,
             venue_id, capacity, created_at, updated_at
             FROM masterclasses WHERE conference_id = ? ORDER BY start_at",
            conference_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }
}

#[async_trait::async_trait]
impl MasterclassInstructorRepository for DbMasterclassInstructorRepository {
    async fn add(
        &self,
        masterclass_id: u64,
        participant_id: u64,
        is_lead: bool,
    ) -> Result<(), DomainError> {
        let now = Utc::now();
        sqlx::query!(
            "INSERT INTO masterclass_instructors
                (masterclass_id, participant_id, is_lead, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)
             ON DUPLICATE KEY UPDATE is_lead = VALUES(is_lead), updated_at = VALUES(updated_at)",
            masterclass_id,
            participant_id,
            is_lead as i8,
            now,
            now,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(())
    }

    async fn remove(
        &self,
        masterclass_id: u64,
        participant_id: u64,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            "DELETE FROM masterclass_instructors
             WHERE masterclass_id = ? AND participant_id = ?",
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
    ) -> Result<Vec<MasterclassInstructorEntity>, DomainError> {
        sqlx::query_as!(
            MasterclassInstructorEntity,
            "SELECT masterclass_id, participant_id, is_lead, created_at, updated_at
             FROM masterclass_instructors WHERE masterclass_id = ?",
            masterclass_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }
}
