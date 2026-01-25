//! Finance system.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::world::World;

/// Finance system.
pub struct FinanceSystem;

impl FinanceSystem {
    /// Run daily finance logic.
    pub fn run_daily(&self, _cfg: &GameConfig, _world: &mut World, _state: &mut GameState) {
        // Stub: process wages, income
    }
}
