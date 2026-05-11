use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::application::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    use argon2::{
        Argon2, PasswordHasher,
        password_hash::{SaltString, rand_core::OsRng},
    };

    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))
        .map(|h| h.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<(), AppError> {
    let parsed = PasswordHash::new(password_hash).map_err(|_| AppError::Unauthorized)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| AppError::Unauthorized)?;

    Ok(())
}
