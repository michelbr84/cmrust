//! Board system (stub).

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::world::World;

/// Board system.
pub struct BoardSystem;

impl BoardSystem {
    pub fn run_daily(&self, _cfg: &GameConfig, _world: &mut World, _state: &mut GameState) {
        // Stub
    }
}
