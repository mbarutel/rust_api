use crate::{
    db_repository,
    domain::{
        error::DomainError,
        models::user::User,
        repository::{Repository, user_repository::UserRepository},
    },
    impl_count, impl_delete,
    infrastructure::database::repository::macros::{map_db_err, map_find_err},
};

db_repository!(DbUserRepository);

// TODO: This should return user entity
#[async_trait::async_trait]
impl Repository<User> for DbUserRepository {
    async fn find_by_id(&self, id: u64) -> Result<User, DomainError> {
        sqlx::query_as!(
            User,
            "SELECT 
                id,
                email,
                first_name,
                last_name,
                password_hash,
                created_at,
                updated_at
            FROM
                users
            WHERE 
                id = ?",
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }

    async fn find_all(&self, offset: u32, limit: u32) -> Result<Vec<User>, DomainError> {
        sqlx::query_as!(
            User,
            "SELECT 
                id,
                first_name,
                last_name,
                email,
                password_hash,
                created_at,
                updated_at
            FROM
                users
            LIMIT ?
            OFFSET ?",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_db_err)
    }

    async fn create(&self, user: User) -> Result<User, DomainError> {
        sqlx::query!(
            "INSERT INTO
                users (
                    first_name,
                    last_name,
                    email,
                    password_hash,
                    created_at,
                    updated_at
                ) 
            VALUES (?, ?, ?, ?, ?, ?)",
            user.first_name,
            user.last_name,
            user.email,
            user.password_hash,
            user.created_at,
            user.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, DomainError> {
        sqlx::query!(
            "UPDATE
                users
            SET
                first_name = ?,
                last_name = ?,
                email = ?,
                password_hash = ?,
                updated_at = ?
            WHERE 
                id = ?",
            user.first_name,
            user.last_name,
            user.email,
            user.password_hash,
            user.updated_at,
            user.id,
        )
        .execute(&self.pool)
        .await
        .map_err(map_db_err)?;

        Ok(user)
    }
    impl_count!("users");
    impl_delete!("users");
}

#[async_trait::async_trait]
impl UserRepository for DbUserRepository {
    async fn email_exists(&self, email: &str) -> Result<bool, DomainError> {
        let exists =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)", email,)
                .fetch_one(&self.pool)
                .await
                .map_err(map_db_err)?;

        Ok(exists == 1)
    }

    async fn find_by_email(&self, email: &str) -> Result<User, DomainError> {
        sqlx::query_as!(
            User,
            "SELECT 
                id,
                email,
                first_name,
                last_name,
                password_hash,
                created_at,
                updated_at
            FROM
                users
            WHERE 
                email = ?",
            email,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_find_err)
    }
}
