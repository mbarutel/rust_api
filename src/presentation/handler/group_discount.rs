use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    application::dto::{
        group_discount::{
            CreateGroupDiscountRequest, GroupDiscountResponse, UpdateGroupDiscountRequest,
        },
        pagination::{ListQueryRequest, PaginatedResponse},
    },
    presentation::{
        error::HandlerError,
        middleware::{auth::AuthUser, validated_json::ValidateJson},
    },
    state::AppState,
};

pub fn group_discount_routes() -> Router<AppState> {
    Router::new()
        .route("/api/group-discounts", get(list).post(create))
        .route(
            "/api/group-discounts/{id}",
            get(find).put(update).delete(delete),
        )
}

async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(query): Query<ListQueryRequest>,
) -> Result<Json<PaginatedResponse<GroupDiscountResponse>>, HandlerError> {
    let (group_discounts, total) = state
        .services
        .group_discount
        .list(query.page, query.per_page)
        .await?;
    let group_discounts = group_discounts
        .into_iter()
        .map(GroupDiscountResponse::from)
        .collect();

    Ok(Json(PaginatedResponse {
        data: group_discounts,
        page: query.page,
        per_page: query.per_page,
        total,
    }))
}

async fn find(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<Json<GroupDiscountResponse>, HandlerError> {
    let group_discount = state.services.group_discount.find_by_id(id).await?;
    Ok(Json(GroupDiscountResponse::from(group_discount)))
}

async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    ValidateJson(dto): ValidateJson<CreateGroupDiscountRequest>,
) -> Result<Json<GroupDiscountResponse>, HandlerError> {
    let group_discount = state.services.group_discount.create(dto).await?;
    Ok(Json(GroupDiscountResponse::from(group_discount)))
}

async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
    ValidateJson(dto): ValidateJson<UpdateGroupDiscountRequest>,
) -> Result<Json<GroupDiscountResponse>, HandlerError> {
    let group_discount = state.services.group_discount.update(id, dto).await?;
    Ok(Json(GroupDiscountResponse::from(group_discount)))
}

async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<u64>,
) -> Result<StatusCode, HandlerError> {
    state.services.group_discount.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use std::sync::Arc;
    use tower::ServiceExt;

    use crate::{
        application::{error::AppError, service::group_discount::MockGroupDiscountService},
        domain::{error::DomainError, models::GroupDiscount},
        presentation::handler::{group_discount::group_discount_routes, utils::test_jwt},
        state::{AppState, Services},
    };

    fn fake_group_discount() -> GroupDiscount {
        GroupDiscount {
            id: 1,
            name: "Register 2 get 1 free".into(),
            min_quantity: 2,
            free_quantity: 1,
            is_active: true,
            valid_until: None,
        }
    }

    fn auth_header() -> String {
        format!("Bearer {}", test_jwt(1))
    }

    fn init_state(group_discount_service: MockGroupDiscountService) -> AppState {
        AppState {
            services: Services {
                group_discount: Arc::new(group_discount_service),
                ..Services::default()
            },
            ..AppState::default()
        }
    }

    #[tokio::test]
    async fn list_ok() {
        let mut group_discount_service = MockGroupDiscountService::new();
        group_discount_service
            .expect_list()
            .once()
            .returning(|_, _| Ok((vec![], 0)));

        let app = group_discount_routes().with_state(init_state(group_discount_service));

        let req = Request::builder()
            .uri("/api/group-discounts")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK)
    }

    #[tokio::test]
    async fn file_not_found() {
        let mut group_discount_service = MockGroupDiscountService::new();
        group_discount_service
            .expect_find_by_id()
            .once()
            .returning(|_| Err(AppError::Domain(DomainError::NotFound)));

        let app = group_discount_routes().with_state(init_state(group_discount_service));

        let req = Request::builder()
            .uri("api/group-discounts/99")
            .header("authorization", auth_header())
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
