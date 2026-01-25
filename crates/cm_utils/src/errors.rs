//! Utility error types.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}
