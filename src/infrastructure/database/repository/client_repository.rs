use crate::{
    application::{
        entity::client_entity::ClientEntity,
        repository::{Repository, client_repository::ClientRepository},
    },
    db_repository,
    domain::error::DomainError,
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbClientRepository);

#[async_trait::async_trait]
impl Repository<ClientEntity> for DbClientRepository {
    async fn find_by_id(&self, id: u64) -> Result<ClientEntity, DomainError> {
        sqlx::query_as!(
            ClientEntity,
            "SELECT
                id,
                organization_id,
                first_name,
                last_name,
                email,
                created_at,
                updated_at
            FROM
                clients
            WHERE
                id = ?",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<ClientEntity>, DomainError> {
        sqlx::query_as!(
            ClientEntity,
            "SELECT
                id,
                organization_id,
                first_name,
                last_name,
                email,
                created_at,
                updated_at
            FROM
                clients
            LIMIT ?
            OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, client: ClientEntity) -> Result<ClientEntity, DomainError> {
        sqlx::query!(
            "INSERT INTO
                clients (
                    organization_id,
                    first_name,
                    last_name,
                    email,
                    created_at,
                    updated_at
                )
            VALUES (?, ?, ?, ?, ?, ?)",
            client.organization_id,
            client.first_name,
            client.last_name,
            client.email,
            client.created_at,
            client.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(client)
    }

    async fn update(&self, client: ClientEntity) -> Result<ClientEntity, DomainError> {
        sqlx::query!(
            "UPDATE
                clients
            SET
                organization_id = ?,
                first_name = ?,
                last_name = ?,
                email = ?,
                updated_at = ?
            WHERE
                id = ?",
            client.organization_id,
            client.first_name,
            client.last_name,
            client.email,
            client.updated_at,
            client.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(client)
    }

    impl_count!("clients");
    impl_delete!("clients");
}

#[async_trait::async_trait]
impl ClientRepository for DbClientRepository {
    async fn find_by_email(&self, email: &str) -> Result<ClientEntity, DomainError> {
        sqlx::query_as!(
            ClientEntity,
            "SELECT
                id,
                organization_id,
                first_name,
                last_name,
                email,
                created_at,
                updated_at
            FROM
                clients
            WHERE
                email = ?",
            email,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn email_exists(&self, email: &str) -> Result<bool, DomainError> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM clients WHERE email = ?)",
            email,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(exists == 1)
    }

    async fn create_in_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        entity: ClientEntity,
    ) -> Result<ClientEntity, DomainError> {
        let result = sqlx::query!(
            "INSERT INTO clients (organization_id, first_name, last_name, email, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            entity.organization_id,
            entity.first_name,
            entity.last_name,
            entity.email,
            entity.created_at,
            entity.updated_at,
        )
        .execute(&mut **tx)
        .await
        .map_err(map_db_err)?;

        Ok(ClientEntity { id: result.last_insert_id(), ..entity })
    }
}
