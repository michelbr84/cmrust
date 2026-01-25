//! Transfer system.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::world::World;
use cm_transfers::window::is_window_open;

/// Transfer system.
pub struct TransferSystem;

impl TransferSystem {
    /// Run daily transfer logic.
    pub fn run_daily(&self, _cfg: &GameConfig, _world: &mut World, state: &mut GameState) {
        state.flags.transfer_window_open = is_window_open(state.date.date());
    }
}
