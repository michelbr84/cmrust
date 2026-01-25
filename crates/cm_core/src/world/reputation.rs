//! Player reputation.

use serde::{Deserialize, Serialize};

/// Reputation level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reputation {
    /// Local reputation (0-100).
    pub local: u8,
    /// National reputation (0-100).
    pub national: u8,
    /// International reputation (0-100).
    pub international: u8,
}

impl Reputation {
    /// Create new reputation.
    pub fn new(local: u8, national: u8, international: u8) -> Self {
        Self {
            local: local.min(100),
            national: national.min(100),
            international: international.min(100),
        }
    }

    /// Get overall reputation.
    pub fn overall(&self) -> u8 {
        ((self.local as u16 + self.national as u16 + self.international as u16) / 3) as u8
    }

    /// Is world class?
    pub fn is_world_class(&self) -> bool {
        self.international >= 80
    }
}

impl Default for Reputation {
    fn default() -> Self {
        Self::new(30, 20, 10)
    }
}
