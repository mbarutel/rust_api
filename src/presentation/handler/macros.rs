/// Generates the `list` handler — paginated GET collection endpoint.
///
/// Handler functions are free-standing `async fn`s (not in a trait impl), so these
/// macros do not need the `Box::pin` workaround required for `#[async_trait]` methods.
///
/// Extractors are accepted as named parameters and unwrapped via `.0` in the body to
/// avoid qualified-path pattern syntax in macros (e.g. `axum::extract::State(s): ...`).
///
/// # Example
/// ```
/// impl_list_handler!(venue_service, VenueResponse);
/// ```
#[macro_export]
macro_rules! impl_list_handler {
    ($service:ident, $response:ty) => {
        async fn list(
            state: ::axum::extract::State<$crate::state::AppState>,
            _auth: $crate::presentation::middleware::auth::AuthUser,
            query: ::axum::extract::Query<
                $crate::application::dto::pagination::ListQueryRequest,
            >,
        ) -> Result<
            ::axum::Json<
                $crate::application::dto::pagination::PaginatedResponse<$response>,
            >,
            $crate::presentation::error::HandlerError,
        > {
            let query = query.0;
            let (items, total) = state.$service.list(query.page, query.per_page).await?;
            let items = items.into_iter().map(<$response>::from).collect();
            Ok(::axum::Json(
                $crate::application::dto::pagination::PaginatedResponse {
                    data: items,
                    page: query.page,
                    per_page: query.per_page,
                    total,
                },
            ))
        }
    };
}

/// Generates the `find` handler — GET single entity by id.
///
/// # Example
/// ```
/// impl_find_handler!(venue_service, VenueResponse);
/// ```
#[macro_export]
macro_rules! impl_find_handler {
    ($service:ident, $response:ty) => {
        async fn find(
            state: ::axum::extract::State<$crate::state::AppState>,
            _auth: $crate::presentation::middleware::auth::AuthUser,
            path: ::axum::extract::Path<u64>,
        ) -> Result<::axum::Json<$response>, $crate::presentation::error::HandlerError> {
            let item = state.$service.find_by_id(path.0).await?;
            Ok(::axum::Json(<$response>::from(item)))
        }
    };
}

/// Generates the `create` handler — POST new entity.
///
/// # Example
/// ```
/// impl_create_handler!(venue_service, CreateVenueRequest, VenueResponse);
/// ```
#[macro_export]
macro_rules! impl_create_handler {
    ($service:ident, $dto:ty, $response:ty) => {
        async fn create(
            state: ::axum::extract::State<$crate::state::AppState>,
            _auth: $crate::presentation::middleware::auth::AuthUser,
            body: $crate::presentation::middleware::validated_json::ValidateJson<$dto>,
        ) -> Result<::axum::Json<$response>, $crate::presentation::error::HandlerError> {
            let item = state.$service.create(body.0).await?;
            Ok(::axum::Json(<$response>::from(item)))
        }
    };
}

/// Generates the `update` handler — PUT existing entity by id.
///
/// # Example
/// ```
/// impl_update_handler!(venue_service, UpdateVenueRequest, VenueResponse);
/// ```
#[macro_export]
macro_rules! impl_update_handler {
    ($service:ident, $dto:ty, $response:ty) => {
        async fn update(
            state: ::axum::extract::State<$crate::state::AppState>,
            _auth: $crate::presentation::middleware::auth::AuthUser,
            path: ::axum::extract::Path<u64>,
            body: $crate::presentation::middleware::validated_json::ValidateJson<$dto>,
        ) -> Result<::axum::Json<$response>, $crate::presentation::error::HandlerError> {
            let item = state.$service.update(path.0, body.0).await?;
            Ok(::axum::Json(<$response>::from(item)))
        }
    };
}

/// Generates the `delete` handler — DELETE entity by id, returns 204.
///
/// # Example
/// ```
/// impl_delete_handler!(venue_service);
/// ```
#[macro_export]
macro_rules! impl_delete_handler {
    ($service:ident) => {
        async fn delete(
            state: ::axum::extract::State<$crate::state::AppState>,
            _auth: $crate::presentation::middleware::auth::AuthUser,
            path: ::axum::extract::Path<u64>,
        ) -> Result<::axum::http::StatusCode, $crate::presentation::error::HandlerError> {
            state.$service.delete(path.0).await?;
            Ok(::axum::http::StatusCode::NO_CONTENT)
        }
    };
}
