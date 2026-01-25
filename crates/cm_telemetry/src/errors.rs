//! Telemetry error types.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("Failed to initialize tracing: {0}")]
    TracingInit(String),

    #[error("Failed to initialize logging: {0}")]
    LoggingInit(String),
}
