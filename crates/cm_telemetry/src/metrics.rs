//! Metrics collection (stub for future implementation).

use std::sync::atomic::{AtomicU64, Ordering};

/// Simple counter metric.
pub struct Counter {
    value: AtomicU64,
}

impl Counter {
    /// Create a new counter.
    pub const fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    /// Increment the counter.
    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    /// Add to the counter.
    pub fn add(&self, n: u64) {
        self.value.fetch_add(n, Ordering::Relaxed);
    }

    /// Get the current value.
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// Game metrics.
pub struct GameMetrics {
    pub matches_simulated: Counter,
    pub days_advanced: Counter,
    pub saves_written: Counter,
}

impl GameMetrics {
    /// Create new game metrics.
    pub const fn new() -> Self {
        Self {
            matches_simulated: Counter::new(),
            days_advanced: Counter::new(),
            saves_written: Counter::new(),
        }
    }
}

impl Default for GameMetrics {
    fn default() -> Self {
        Self::new()
    }
}
