//! # CM Telemetry
//!
//! Logging, tracing, and metrics for the CM game.

pub mod errors;
pub mod logging;
pub mod metrics;
pub mod tracing;

pub use crate::tracing::init_tracing;
