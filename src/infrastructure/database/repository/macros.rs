use crate::domain::error::DomainError;

pub fn map_db_err(e: sqlx::Error) -> DomainError {
    DomainError::Database(e.to_string())
}

pub fn map_find_err(e: sqlx::Error) -> DomainError {
    match e {
        sqlx::Error::RowNotFound => DomainError::NotFound,
        _ => DomainError::Database(e.to_string()),
    }
}

/// Generates a repository struct with a `pool: MySqlPool` field and a `new()` constructor.
///
/// # Example
/// ```
/// db_repository!(DbVenueRepository);
/// ```
#[macro_export]
macro_rules! db_repository {
    ($name:ident) => {
        pub struct $name {
            pool: ::sqlx::MySqlPool,
        }

        impl $name {
            pub fn new(pool: ::sqlx::MySqlPool) -> Self {
                Self { pool }
            }
        }
    };
}

/// Generates the `find_by_id` method for a `Repository<T>` impl block.
/// The `id` parameter name is owned by this macro.
///
/// # Example
/// ```
/// impl_find_by_id!(User, "SELECT id, email FROM users WHERE id = ?");
/// ```
#[macro_export]
macro_rules! impl_find_by_id {
    ($entity:ty, $sql:literal) => {
        fn find_by_id<'life0, 'async_trait>(
            &'life0 self,
            id: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<$entity, $crate::domain::error::DomainError>,
                > + ::core::marker::Send
                  + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                sqlx::query_as!($entity, $sql, id)
                    .fetch_one(&self.pool)
                    .await
                    .map_err($crate::infrastructure::database::repository::macros::map_find_err)
            })
        }
    };
}

/// Generates the `find_all` method for a `Repository<T>` impl block.
/// The SQL must bind `LIMIT ? OFFSET ?` in that order.
/// The `limit` and `offset` parameter names are owned by this macro.
///
/// # Example
/// ```
/// impl_find_all!(User, "SELECT id, email FROM users LIMIT ? OFFSET ?");
/// ```
#[macro_export]
macro_rules! impl_find_all {
    ($entity:ty, $sql:literal) => {
        fn find_all<'life0, 'async_trait>(
            &'life0 self,
            offset: u32,
            limit: u32,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Vec<$entity>, $crate::domain::error::DomainError>,
                > + ::core::marker::Send
                  + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                sqlx::query_as!($entity, $sql, limit, offset)
                    .fetch_all(&self.pool)
                    .await
                    .map_err($crate::infrastructure::database::repository::macros::map_db_err)
            })
        }
    };
}

/// Generates the `create` method for a `Repository<T>` impl block.
/// The entity parameter is named `entity` — field bindings must use that name.
///
/// # Example
/// ```
/// impl_create!(
///     User,
///     "INSERT INTO users (first_name, email) VALUES (?, ?)",
///     entity.first_name,
///     entity.email,
/// );
/// ```
#[macro_export]
macro_rules! impl_create {
    ($entity:ty, $sql:literal, $($field:expr),+ $(,)?) => {
        fn create<'life0, 'async_trait>(
            &'life0 self,
            entity: $entity,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<$entity, $crate::domain::error::DomainError>,
                > + ::core::marker::Send
                  + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                sqlx::query!($sql, $($field),+)
                    .execute(&self.pool)
                    .await
                    .map_err($crate::infrastructure::database::repository::macros::map_db_err)?;
                Ok(entity)
            })
        }
    };
}

/// Generates the `update` method for a `Repository<T>` impl block.
/// The entity parameter is named `entity` — field bindings must use that name.
///
/// # Example
/// ```
/// impl_update!(
///     User,
///     "UPDATE users SET first_name = ?, email = ? WHERE id = ?",
///     entity.first_name,
///     entity.email,
///     entity.id,
/// );
/// ```
#[macro_export]
macro_rules! impl_update {
    ($entity:ty, $sql:literal, $($field:expr),+ $(,)?) => {
        fn update<'life0, 'async_trait>(
            &'life0 self,
            entity: $entity,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<$entity, $crate::domain::error::DomainError>,
                > + ::core::marker::Send
                  + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                sqlx::query!($sql, $($field),+)
                    .execute(&self.pool)
                    .await
                    .map_err($crate::infrastructure::database::repository::macros::map_db_err)?;
                Ok(entity)
            })
        }
    };
}

/// Generates the `delete` method for a `Repository<T>` impl block.
///
/// Uses the runtime `sqlx::query()` (no `!`) so `concat!` works for the table name.
/// Emits the `Box::pin` signature manually because `#[async_trait]` is an attribute
/// proc macro that runs before `macro_rules!` expansions — methods generated inside
/// a macro would otherwise miss the lifetime transformation.
///
/// # Example
/// ```
/// impl Repository<Venue> for DbVenueRepository {
///     impl_delete!("venues");
/// }
/// ```
#[macro_export]
macro_rules! impl_delete {
    ($table:literal) => {
        fn delete<'life0, 'async_trait>(
            &'life0 self,
            id: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<(), $crate::domain::error::DomainError>,
                > + ::core::marker::Send
                  + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                sqlx::query(concat!("DELETE FROM ", $table, " WHERE id = ?"))
                    .bind(id)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| $crate::domain::error::DomainError::Database(e.to_string()))?;
                Ok(())
            })
        }
    };
}

/// Generates the `count` method for a `Repository<T>` impl block.
///
/// See `impl_delete!` for why `Box::pin` is used directly instead of `async fn`.
///
/// # Example
/// ```
/// impl Repository<Venue> for DbVenueRepository {
///     impl_count!("venues");
/// }
/// ```
#[macro_export]
macro_rules! impl_count {
    ($table:literal) => {
        fn count<'life0, 'async_trait>(
            &'life0 self,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<u64, $crate::domain::error::DomainError>,
                > + ::core::marker::Send
                  + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                let count: i64 =
                    sqlx::query_scalar(concat!("SELECT COUNT(*) FROM ", $table))
                        .fetch_one(&self.pool)
                        .await
                        .map_err(|e| $crate::domain::error::DomainError::Database(e.to_string()))?;
                Ok(count as u64)
            })
        }
    };
}
