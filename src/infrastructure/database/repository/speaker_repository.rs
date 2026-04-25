use crate::{
    application::{
        entity::speaker_entity::SpeakerEntity,
        repository::{Repository, speaker_repository::SpeakerRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbSpeakerRepository);

#[async_trait::async_trait]
impl Repository<SpeakerEntity> for DbSpeakerRepository {
    async fn find_by_id(&self, id: u64) -> Result<SpeakerEntity, DomainError> {
        sqlx::query_as!(
            SpeakerEntity,
            "SELECT id, participant_id, talk_title, talk_abstract, duration_minutes,
             av_requirements, headshot, bio, created_at, updated_at
             FROM speakers WHERE id = ?",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<SpeakerEntity>, DomainError> {
        sqlx::query_as!(
            SpeakerEntity,
            "SELECT id, participant_id, talk_title, talk_abstract, duration_minutes,
             av_requirements, headshot, bio, created_at, updated_at
             FROM speakers LIMIT ? OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, entity: SpeakerEntity) -> Result<SpeakerEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO speakers (
                participant_id, talk_title, talk_abstract, duration_minutes,
                av_requirements, headshot, bio, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            entity.participant_id,
            entity.talk_title,
            entity.talk_abstract,
            entity.duration_minutes,
            entity.av_requirements,
            entity.headshot,
            entity.bio,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    async fn update(&self, entity: SpeakerEntity) -> Result<SpeakerEntity, DomainError> {
        sqlx::query!(
            "UPDATE speakers SET
                talk_title = ?, talk_abstract = ?, duration_minutes = ?,
                av_requirements = ?, headshot = ?, bio = ?, updated_at = ?
             WHERE id = ?",
            entity.talk_title,
            entity.talk_abstract,
            entity.duration_minutes,
            entity.av_requirements,
            entity.headshot,
            entity.bio,
            entity.updated_at,
            entity.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    impl_count!("speakers");
    impl_delete!("speakers");
}

#[async_trait::async_trait]
impl SpeakerRepository for DbSpeakerRepository {
    async fn find_by_participant_id(
        &self,
        participant_id: u64,
    ) -> Result<SpeakerEntity, DomainError> {
        sqlx::query_as!(
            SpeakerEntity,
            "SELECT id, participant_id, talk_title, talk_abstract, duration_minutes,
             av_requirements, headshot, bio, created_at, updated_at
             FROM speakers WHERE participant_id = ?",
            participant_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }
}
