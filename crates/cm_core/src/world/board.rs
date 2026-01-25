//! Board entity (club management).

use serde::{Deserialize, Serialize};

/// Club board expectations and confidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    /// Board confidence in manager (0-100).
    pub confidence: u8,
    /// Expected league position.
    pub expected_position: u8,
    /// Whether board expects cup run.
    pub expects_cup_run: bool,
    /// Financial strictness (0-100).
    pub financial_strictness: u8,
    /// Youth focus (0-100).
    pub youth_focus: u8,
}

impl Board {
    /// Create a new board with default expectations.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if board is happy.
    pub fn is_happy(&self) -> bool {
        self.confidence >= 50
    }

    /// Adjust confidence.
    pub fn adjust_confidence(&mut self, delta: i8) {
        let new = (self.confidence as i16 + delta as i16).clamp(0, 100);
        self.confidence = new as u8;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            confidence: 70,
            expected_position: 10,
            expects_cup_run: false,
            financial_strictness: 50,
            youth_focus: 50,
        }
    }
}
