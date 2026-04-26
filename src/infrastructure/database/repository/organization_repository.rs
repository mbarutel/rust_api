use crate::{
    application::{
        entity::organization_entity::OrganizationEntity,
        repository::{Repository, organization_repository::OrganizationRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbOrganizationRepository);

#[async_trait::async_trait]
impl Repository<OrganizationEntity> for DbOrganizationRepository {
    async fn find_by_id(&self, id: u64) -> Result<OrganizationEntity, DomainError> {
        sqlx::query_as!(
            OrganizationEntity,
            "SELECT 
                id,
                name,
                website,
                phone,
                billing_email,
                created_at,
                updated_at
            FROM
                organizations
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
    ) -> Result<Vec<OrganizationEntity>, DomainError> {
        sqlx::query_as!(
            OrganizationEntity,
            "SELECT
                id,
                name,
                website,
                phone,
                billing_email,
                created_at,
                updated_at
            FROM
                organizations
            LIMIT ?
            OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(
        &self,
        organization: OrganizationEntity,
    ) -> Result<OrganizationEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO
                organizations (
                    name,
                    website,
                    phone,
                    billing_email,
                    created_at,
                    updated_at
                )
            VALUES (?, ?, ?, ?, ?, ?)",
            organization.name,
            organization.website,
            organization.phone,
            organization.billing_email,
            organization.created_at,
            organization.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(organization)
    }

    async fn update(
        &self,
        organization: OrganizationEntity,
    ) -> Result<OrganizationEntity, DomainError> {
        sqlx::query!(
            "UPDATE
                organizations
            SET
                name = ?,
                website = ?,
                phone = ?,
                billing_email = ?,
                updated_at = ?
            WHERE
                id = ?",
            organization.name,
            organization.website,
            organization.phone,
            organization.billing_email,
            organization.updated_at,
            organization.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(organization)
    }

    impl_count!("organizations");
    impl_delete!("organizations");
}

#[async_trait::async_trait]
impl OrganizationRepository for DbOrganizationRepository {
    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: OrganizationEntity,
    ) -> Result<OrganizationEntity, DomainError> {
        let result = sqlx::query!(
            "INSERT INTO organizations (name, website, phone, billing_email, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            entity.name,
            entity.website,
            entity.phone,
            entity.billing_email,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&mut **tx)
        .await
        .map_err(map_db_err)?;

        Ok(OrganizationEntity { id: result.last_insert_id(), ..entity })
    }
}
