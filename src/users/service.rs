use sqlx::MySqlPool;

use crate::{
    error::{AppError, Result},
    users::{
        model::{CreateUserRequest, UserResponse},
        repository,
    },
};

pub async fn create_user(pool: &MySqlPool, payload: CreateUserRequest) -> Result<UserResponse> {
    // Check for email uniqueness
    if repository::email_exists(pool, &payload.email).await? {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Hash password
    let password_hash = hash_password(&payload.password)?;
    //
    // Insert user
    let now = chrono::Utc::now();
    let id = repository::insert(pool, &payload.email, &payload.name, &password_hash, now).await?;

    // Send welcome email (future)
    // email::send_welcome(&payload.email).await?;

    // Create audit log (future)
    // audit::log("user_created", id).await?;

    Ok(UserResponse {
        id,
        email: payload.email,
        name: payload.name,
        created_at: now,
        updated_at: now,
    })
}

fn hash_password(password: &str) -> Result<String> {
    use argon2::{
        Argon2, PasswordHasher,
        password_hash::{SaltString, rand_core::OsRng},
    };

    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::Error::msg(e.to_string())))
        .map(|h| h.to_string())
}
