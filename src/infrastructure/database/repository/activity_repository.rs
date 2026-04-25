use crate::{
    application::{
        entity::activity_entity::ActivityEntity,
        repository::{Repository, activity_repository::ActivityRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbActivityRepository);

#[async_trait::async_trait]
impl Repository<ActivityEntity> for DbActivityRepository {
    async fn find_by_id(&self, id: u64) -> Result<ActivityEntity, DomainError> {
        sqlx::query_as!(
            ActivityEntity,
            "SELECT id, conference_id, name, description, start_at, end_at,
             venue_id, provider_url, capacity, created_at, updated_at
             FROM activities WHERE id = ?",
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
    ) -> Result<Vec<ActivityEntity>, DomainError> {
        sqlx::query_as!(
            ActivityEntity,
            "SELECT id, conference_id, name, description, start_at, end_at,
             venue_id, provider_url, capacity, created_at, updated_at
             FROM activities LIMIT ? OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, entity: ActivityEntity) -> Result<ActivityEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO activities (
                conference_id, name, description, start_at, end_at,
                venue_id, provider_url, capacity, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            entity.conference_id,
            entity.name,
            entity.description,
            entity.start_at,
            entity.end_at,
            entity.venue_id,
            entity.provider_url,
            entity.capacity,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    async fn update(&self, entity: ActivityEntity) -> Result<ActivityEntity, DomainError> {
        sqlx::query!(
            "UPDATE activities SET
                name = ?, description = ?, start_at = ?, end_at = ?,
                venue_id = ?, provider_url = ?, capacity = ?, updated_at = ?
             WHERE id = ?",
            entity.name,
            entity.description,
            entity.start_at,
            entity.end_at,
            entity.venue_id,
            entity.provider_url,
            entity.capacity,
            entity.updated_at,
            entity.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    impl_count!("activities");
    impl_delete!("activities");
}

#[async_trait::async_trait]
impl ActivityRepository for DbActivityRepository {
    async fn find_by_conference(
        &self,
        conference_id: u64,
    ) -> Result<Vec<ActivityEntity>, DomainError> {
        sqlx::query_as!(
            ActivityEntity,
            "SELECT id, conference_id, name, description, start_at, end_at,
             venue_id, provider_url, capacity, created_at, updated_at
             FROM activities WHERE conference_id = ? ORDER BY start_at",
            conference_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }
}
