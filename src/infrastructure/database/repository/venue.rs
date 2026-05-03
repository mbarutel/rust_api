use crate::{
    application::{
        entity::venue::VenueEntity,
        repository::{Repository, venue::VenueRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbVenueRepository);

#[async_trait::async_trait]
impl Repository<VenueEntity> for DbVenueRepository {
    async fn find_by_id(&self, id: u64) -> Result<VenueEntity, DomainError> {
        sqlx::query_as!(
            VenueEntity,
            "SELECT id, name, address_line1, address_line2, city, state_region,
             postal_code, country, notes, created_at, updated_at
             FROM venues WHERE id = ?",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<VenueEntity>, DomainError> {
        sqlx::query_as!(
            VenueEntity,
            "SELECT id, name, address_line1, address_line2, city, state_region,
             postal_code, country, notes, created_at, updated_at
             FROM venues LIMIT ? OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, venue: VenueEntity) -> Result<VenueEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO venues (
                name, address_line1, address_line2, city, state_region,
                postal_code, country, notes, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            venue.name,
            venue.address_line1,
            venue.address_line2,
            venue.city,
            venue.state_region,
            venue.postal_code,
            venue.country,
            venue.notes,
            venue.created_at,
            venue.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(venue)
    }

    async fn update(&self, venue: VenueEntity) -> Result<VenueEntity, DomainError> {
        sqlx::query!(
            "UPDATE venues SET
                name = ?, address_line1 = ?, address_line2 = ?, city = ?,
                state_region = ?, postal_code = ?, country = ?, notes = ?,
                updated_at = ?
             WHERE id = ?",
            venue.name,
            venue.address_line1,
            venue.address_line2,
            venue.city,
            venue.state_region,
            venue.postal_code,
            venue.country,
            venue.notes,
            venue.updated_at,
            venue.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(venue)
    }

    impl_count!("venues");
    impl_delete!("venues");
}

impl VenueRepository for DbVenueRepository {}
