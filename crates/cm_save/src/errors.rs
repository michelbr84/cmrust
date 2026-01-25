//! Save error types.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SaveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Integrity error: hash mismatch")]
    IntegrityError,

    #[error("Version mismatch: save version {save} != current {current}")]
    VersionMismatch { save: u32, current: u32 },

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Save not found: {0}")]
    NotFound(String),
}
