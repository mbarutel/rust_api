use crate::{
    application::{
        entity::conference_entity::ConferenceEntity,
        repository::{Repository, conference_repository::ConferenceRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbConferenceRepository);

#[async_trait::async_trait]
impl Repository<ConferenceEntity> for DbConferenceRepository {
    async fn find_by_id(&self, id: u64) -> Result<ConferenceEntity, DomainError> {
        sqlx::query_as!(
            ConferenceEntity,
            "SELECT 
                id,
                code,
                name,
                poster_url,
                description,
                start_date,
                end_date,
                venue_id,
                group_discount_id,
                published,
                created_at,
                updated_at
            FROM
                conferences
            WHERE 
                id = ?",
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
    ) -> Result<Vec<ConferenceEntity>, DomainError> {
        sqlx::query_as!(
            ConferenceEntity,
            "SELECT
                id,
                code,
                name,
                poster_url,
                description,
                start_date,
                end_date,
                venue_id,
                group_discount_id,
                published,
                created_at,
                updated_at
            FROM
                conferences
            LIMIT ?
            OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, conference: ConferenceEntity) -> Result<ConferenceEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO
                conferences (
                    code,
                    name,
                    poster_url,
                    description,
                    start_date,
                    end_date,
                    venue_id,
                    group_discount_id,
                    published,
                    created_at,
                    updated_at
                )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )",
            conference.code,
            conference.name,
            conference.poster_url,
            conference.description,
            conference.start_date,
            conference.end_date,
            conference.venue_id,
            conference.group_discount_id,
            conference.published,
            conference.created_at,
            conference.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(conference)
    }

    async fn update(&self, conference: ConferenceEntity) -> Result<ConferenceEntity, DomainError> {
        sqlx::query!(
            "UPDATE
                conferences
            SET
                name = ?,
                poster_url = ?,
                description = ?,
                start_date = ?,
                end_date = ?,
                venue_id = ?,
                group_discount_id = ?,
                published = ?,
                updated_at = ?
            WHERE
                id = ?",
            conference.name,
            conference.poster_url,
            conference.description,
            conference.start_date,
            conference.end_date,
            conference.venue_id,
            conference.group_discount_id,
            conference.published,
            conference.updated_at,
            conference.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(conference)
    }

    impl_count!("conferences");
    impl_delete!("conferences");
}

#[async_trait::async_trait]
impl ConferenceRepository for DbConferenceRepository {
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: ConferenceEntity,
    ) -> Result<ConferenceEntity, DomainError> {
        let result = sqlx::query!(
            "INSERT INTO
                conferences (
                    code,
                    name,
                    poster_url,
                    description,
                    start_date,
                    end_date,
                    venue_id,
                    group_discount_id,
                    published,
                    created_at,
                    updated_at
                )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )",
            entity.code,
            entity.name,
            entity.poster_url,
            entity.description,
            entity.start_date,
            entity.end_date,
            entity.venue_id,
            entity.group_discount_id,
            entity.published,
            entity.created_at,
            entity.updated_at
        )
        .execute(&mut **tx)
        .await
        .map_err(map_db_err)?;

        Ok(ConferenceEntity {
            id: result.last_insert_id(),
            ..entity
        })
    }
}
