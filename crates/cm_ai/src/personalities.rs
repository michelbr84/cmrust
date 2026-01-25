//! AI personality types.

use serde::{Deserialize, Serialize};

/// Manager personality affects AI decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManagerPersonality {
    Defensive,
    Balanced,
    Attacking,
    YouthFocused,
    WinAtAllCosts,
    Financial,
}

impl Default for ManagerPersonality {
    fn default() -> Self {
        Self::Balanced
    }
}

/// Get squad depth preference.
pub fn preferred_squad_size(personality: ManagerPersonality) -> usize {
    match personality {
        ManagerPersonality::Defensive => 28,
        ManagerPersonality::Balanced => 25,
        ManagerPersonality::Attacking => 22,
        ManagerPersonality::YouthFocused => 30,
        ManagerPersonality::WinAtAllCosts => 25,
        ManagerPersonality::Financial => 22,
    }
}

/// Get youth development priority.
pub fn youth_priority(personality: ManagerPersonality) -> u8 {
    match personality {
        ManagerPersonality::YouthFocused => 90,
        ManagerPersonality::Financial => 70,
        ManagerPersonality::Balanced => 50,
        ManagerPersonality::Attacking => 40,
        ManagerPersonality::Defensive => 40,
        ManagerPersonality::WinAtAllCosts => 20,
    }
}
