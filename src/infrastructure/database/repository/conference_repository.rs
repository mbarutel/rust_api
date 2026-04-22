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

    async fn create(&self, venue: ConferenceEntity) -> Result<ConferenceEntity, DomainError> {
        todo!();
        // sqlx::query!(
        //     "INSERT INTO
        //         conferences (
        //             name,
        //             address_line1,
        //             address_line2,
        //             city,
        //             state_region,
        //             postal_code,
        //             country,
        //             notes,
        //             published,
        //             created_at,
        //             updated_at
        //         )
        //     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        //     venue.name,
        //     venue.address_line1,
        //     venue.address_line2,
        //     venue.city,
        //     venue.state_region,
        //     venue.postal_code,
        //     venue.country,
        //     venue.notes,
        //     venue.published,
        //     venue.created_at,
        //     venue.updated_at,
        // )
        // .execute(&self.pool)
        // .await
        // .map_err(map_db_err)?;

        // Ok(venue)
    }

    async fn update(&self, venue: ConferenceEntity) -> Result<ConferenceEntity, DomainError> {
        todo!();
        // sqlx::query!(
        //     "UPDATE
        //         conferences
        //     SET
        //         name = ?,
        //         address_line1 = ?,
        //         address_line2 = ?,
        //         city = ?,
        //         state_region = ?,
        //         postal_code = ?,
        //         country = ?,
        //         notes = ?,
        //         published = ?,
        //         updated_at = ?
        //     WHERE
        //         id = ?",
        //     venue.name,
        //     venue.address_line1,
        //     venue.address_line2,
        //     venue.city,
        //     venue.state_region,
        //     venue.postal_code,
        //     venue.country,
        //     venue.notes,
        //     venue.published,
        //     venue.updated_at,
        //     venue.id,
        // )
        // .execute(&self.pool)
        // .await
        // .map_err(map_db_err)?;

        // Ok(venue)
    }

    impl_count!("conferences");
    impl_delete!("conferences");
}

impl ConferenceRepository for DbConferenceRepository {}
