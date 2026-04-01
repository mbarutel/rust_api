// mod common;
//
// use axum::http::StatusCode;
// use serde_json::Value;
// use tower::ServiceExt;
//
// #[tokio::test]
// async fn test_unauthenticated_request_returns_401() {
//     use axum::{body::Body, http::Request};
//
//     let app = common::build_test_app().await;
//
//     // Request with no Authorization header
//     let response = app
//         .oneshot(
//             Request::builder()
//                 .uri("/api/users")
//                 .body(Body::empty())
//                 .unwrap(),
//         )
//         .await
//         .unwrap();
//
//     assert_eq!(response.status(), StatusCode::UNAUTHORIZED)
// }
//
// #[tokio::test]
// async fn test_create_user() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_create_user_duplicate_email() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_create_user_invalid_email() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_create_user_short_password() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_create_user_empty_name() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_create_user_missing_fields() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_create_user_invalid_json() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_get_user() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_get_user_not_found() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_update_user_name() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_update_user_email() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_update_user_empty_body() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_update_user_not_found() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_update_user_invalid_email() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_delete_user() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_delete_user_not_found() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_list_users_empty() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_list_users_returns_created_users() {
//     todo!()
// }
//
// #[tokio::test]
// async fn test_list_users_pagination() {
//     todo!()
// }
