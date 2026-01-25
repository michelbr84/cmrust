//! Logging utilities.

use tracing::{debug, error, info, trace, warn};

/// Log levels for convenience re-exports.
pub use tracing::{Level, Span};

/// Log an info message.
#[inline]
pub fn log_info(msg: &str) {
    info!("{}", msg);
}

/// Log a debug message.
#[inline]
pub fn log_debug(msg: &str) {
    debug!("{}", msg);
}

/// Log a warning message.
#[inline]
pub fn log_warn(msg: &str) {
    warn!("{}", msg);
}

/// Log an error message.
#[inline]
pub fn log_error(msg: &str) {
    error!("{}", msg);
}

/// Log a trace message.
#[inline]
pub fn log_trace(msg: &str) {
    trace!("{}", msg);
}
