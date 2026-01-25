//! Engine error types.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Game not initialized")]
    NotInitialized,

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("System error: {0}")]
    System(String),

    #[error("Save error: {0}")]
    Save(#[from] cm_save::SaveError),

    #[error("Core error: {0}")]
    Core(#[from] cm_core::CoreError),
}
