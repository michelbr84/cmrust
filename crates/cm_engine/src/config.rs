//! Game configuration.

use serde::{Deserialize, Serialize};
use cm_core::sim::GameRules;

/// Game configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub rules: GameRules,
    pub difficulty: u8,
    pub auto_save: bool,
    pub auto_save_interval: u16, // days
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            rules: GameRules::default(),
            difficulty: 50,
            auto_save: true,
            auto_save_interval: 7,
        }
    }
}
