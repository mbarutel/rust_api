#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("enitity not found")]
    NotFound,
    #[error("entity already exists")]
    Conflict,
    #[error("database error: {0}")]
    Database(String),
}
