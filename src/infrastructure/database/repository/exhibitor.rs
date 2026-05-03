use crate::{
    application::{
        entity::exhibitor::ExhibitorEntity,
        repository::{Repository, exhibitor::ExhibitorRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbExhibitorRepository);

#[async_trait::async_trait]
impl Repository<ExhibitorEntity> for DbExhibitorRepository {
    async fn find_by_id(&self, id: u64) -> Result<ExhibitorEntity, DomainError> {
        sqlx::query_as!(
            ExhibitorEntity,
            "SELECT id, participant_id, company_name, power_required, internet_required,
             notes_internal, created_at, updated_at
             FROM exhibitors WHERE id = ?",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<ExhibitorEntity>, DomainError> {
        sqlx::query_as!(
            ExhibitorEntity,
            "SELECT id, participant_id, company_name, power_required, internet_required,
             notes_internal, created_at, updated_at
             FROM exhibitors LIMIT ? OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, entity: ExhibitorEntity) -> Result<ExhibitorEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO exhibitors (
                participant_id, company_name, power_required, internet_required,
                notes_internal, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            entity.participant_id,
            entity.company_name,
            entity.power_required,
            entity.internet_required,
            entity.notes_internal,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    async fn update(&self, entity: ExhibitorEntity) -> Result<ExhibitorEntity, DomainError> {
        sqlx::query!(
            "UPDATE exhibitors SET
                company_name = ?, power_required = ?, internet_required = ?,
                notes_internal = ?, updated_at = ?
             WHERE id = ?",
            entity.company_name,
            entity.power_required,
            entity.internet_required,
            entity.notes_internal,
            entity.updated_at,
            entity.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;
        Ok(entity)
    }

    impl_count!("exhibitors");
    impl_delete!("exhibitors");
}

#[async_trait::async_trait]
impl ExhibitorRepository for DbExhibitorRepository {
    async fn find_by_participant_id(
        &self,
        participant_id: u64,
    ) -> Result<ExhibitorEntity, DomainError> {
        sqlx::query_as!(
            ExhibitorEntity,
            "SELECT id, participant_id, company_name, power_required, internet_required,
             notes_internal, created_at, updated_at
             FROM exhibitors WHERE participant_id = ?",
            participant_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: ExhibitorEntity,
    ) -> Result<ExhibitorEntity, DomainError> {
        let result = sqlx::query!(
            "INSERT INTO exhibitors (
                participant_id, company_name, power_required, internet_required,
                notes_internal, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            entity.participant_id,
            entity.company_name,
            entity.power_required,
            entity.internet_required,
            entity.notes_internal,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&mut **tx)
        .await
        .map_err(map_db_err)?;

        Ok(ExhibitorEntity {
            id: result.last_insert_id(),
            ..entity
        })
    }
}
