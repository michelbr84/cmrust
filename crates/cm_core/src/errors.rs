//! Core error types.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: String, id: String },

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Date parse error: {0}")]
    DateParse(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
