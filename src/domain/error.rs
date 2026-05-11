#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("enitity not found")]
    NotFound,
    #[error("entity already exists")]
    Conflict,
    #[error("invalid status transition: {0}")]
    InvalidTransition(String),
    #[error("database error: {0}")]
    Database(String),
}
