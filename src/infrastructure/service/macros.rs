/// Generates a service struct with a single repository field and a `new()` constructor.
/// Mirrors `db_repository!` in the database layer.
///
/// # Example
/// ```
/// impl_service!(VenueServiceImpl, venue_repo, VenueRepository);
/// ```
#[macro_export]
macro_rules! impl_service {
    ($name:ident, $field:ident, $repo:path) => {
        pub struct $name {
            $field: ::std::sync::Arc<dyn $repo>,
        }

        impl $name {
            pub fn new($field: ::std::sync::Arc<dyn $repo>) -> Self {
                Self { $field }
            }
        }
    };
}

/// Generates the `list` method for a service impl block.
/// Computes pagination offset from `page`/`per_page` and delegates to the repository.
///
/// Uses `Box::pin` directly because `#[async_trait]` is a proc macro that runs before
/// `macro_rules!` expansions — methods generated inside a macro would otherwise miss
/// the lifetime transformation. See `impl_delete!` in the repository macros for details.
///
/// # Example
/// ```
/// impl VenueService for VenueServiceImpl {
///     impl_service_list!(venue_repo, Venue);
/// }
/// ```
#[macro_export]
macro_rules! impl_service_list {
    ($field:ident, $entity:ty) => {
        fn list<'life0, 'async_trait>(
            &'life0 self,
            page: u32,
            per_page: u32,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<
                        (Vec<$entity>, u64),
                        $crate::application::error::AppError,
                    >,
                > + ::core::marker::Send
                  + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                let offset = (page - 1) * per_page;
                let total = self.$field.count().await?;
                let items = self.$field.find_all(offset, per_page).await?;
                Ok((items, total))
            })
        }
    };
}

/// Generates the `find_by_id` method for a service impl block.
/// See `impl_service_list!` for why `Box::pin` is used.
///
/// # Example
/// ```
/// impl VenueService for VenueServiceImpl {
///     impl_service_find_by_id!(venue_repo, Venue);
/// }
/// ```
#[macro_export]
macro_rules! impl_service_find_by_id {
    ($field:ident, $entity:ty) => {
        fn find_by_id<'life0, 'async_trait>(
            &'life0 self,
            id: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<$entity, $crate::application::error::AppError>,
                > + ::core::marker::Send
                  + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move { Ok(self.$field.find_by_id(id).await?) })
        }
    };
}

/// Generates the `delete` method for a service impl block.
/// See `impl_service_list!` for why `Box::pin` is used.
///
/// # Example
/// ```
/// impl VenueService for VenueServiceImpl {
///     impl_service_delete!(venue_repo);
/// }
/// ```
#[macro_export]
macro_rules! impl_service_delete {
    ($field:ident) => {
        fn delete<'life0, 'async_trait>(
            &'life0 self,
            id: u64,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<(), $crate::application::error::AppError>,
                > + ::core::marker::Send
                  + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move { Ok(self.$field.delete(id).await?) })
        }
    };
}
