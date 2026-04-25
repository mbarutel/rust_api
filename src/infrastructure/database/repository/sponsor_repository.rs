use crate::{
    application::{
        entity::sponsor_entity::SponsorEntity,
        repository::{Repository, sponsor_repository::SponsorRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbSponsorRepository);

#[async_trait::async_trait]
impl Repository<SponsorEntity> for DbSponsorRepository {
    async fn find_by_id(&self, id: u64) -> Result<SponsorEntity, DomainError> {
        sqlx::query_as!(
            SponsorEntity,
            "SELECT id, participant_id, tier, company_name, logo_url,
             invoice_contact, benefits_notes, created_at, updated_at
             FROM sponsors WHERE id = ?",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<SponsorEntity>, DomainError> {
        sqlx::query_as!(
            SponsorEntity,
            "SELECT id, participant_id, tier, company_name, logo_url,
             invoice_contact, benefits_notes, created_at, updated_at
             FROM sponsors LIMIT ? OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, entity: SponsorEntity) -> Result<SponsorEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO sponsors (
                participant_id, tier, company_name, logo_url,
                invoice_contact, benefits_notes, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            entity.participant_id,
            entity.tier,
            entity.company_name,
            entity.logo_url,
            entity.invoice_contact,
            entity.benefits_notes,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    async fn update(&self, entity: SponsorEntity) -> Result<SponsorEntity, DomainError> {
        sqlx::query!(
            "UPDATE sponsors SET
                tier = ?, company_name = ?, logo_url = ?,
                invoice_contact = ?, benefits_notes = ?, updated_at = ?
             WHERE id = ?",
            entity.tier,
            entity.company_name,
            entity.logo_url,
            entity.invoice_contact,
            entity.benefits_notes,
            entity.updated_at,
            entity.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    impl_count!("sponsors");
    impl_delete!("sponsors");
}

#[async_trait::async_trait]
impl SponsorRepository for DbSponsorRepository {
    async fn find_by_participant_id(
        &self,
        participant_id: u64,
    ) -> Result<SponsorEntity, DomainError> {
        sqlx::query_as!(
            SponsorEntity,
            "SELECT id, participant_id, tier, company_name, logo_url,
             invoice_contact, benefits_notes, created_at, updated_at
             FROM sponsors WHERE participant_id = ?",
            participant_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }
}
