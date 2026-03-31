use axum::Router;

pub async fn build_test_app() -> Router {
    dotenvy::dotenv().ok();
    let config = rust_api::config::Config::from_env();
    let state = rust_api::state::AppState::new(&config)
        .await
        .expect("Failed to create test app state");

    sqlx::migrate!()
        .run(&state.db)
        .await
        .expect("Failed to run migrations");

    sqlx::query("DELETE FROM users")
        .execute(&state.db)
        .await
        .expect("Failed to clean users table");

    rust_api::build_router(state)
}
