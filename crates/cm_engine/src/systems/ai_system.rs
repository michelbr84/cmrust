//! AI system.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::world::World;

/// AI system for CPU clubs.
pub struct AiSystem;

impl AiSystem {
    /// Run daily AI.
    pub fn run_daily(&self, _cfg: &GameConfig, _world: &mut World, _state: &mut GameState) {
        // Stub: AI decisions for CPU clubs
    }
}
