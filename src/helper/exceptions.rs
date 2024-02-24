use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RepositoryError {
    NotFound(String),
    InternalServerError(String),
    Conflict(String),
    Unknown(String),
}

impl From<diesel::result::Error> for RepositoryError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => RepositoryError::NotFound(e.to_string()),
            _ => RepositoryError::InternalServerError(e.to_string()),
        }
    }
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound(e) => write!(f, "Not found: {}", e),
            RepositoryError::InternalServerError(e) => write!(f, "Internal server error: {}", e),
            RepositoryError::Conflict(e) => write!(f, "Conflict: {}", e),
            RepositoryError::Unknown(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for RepositoryError {}
