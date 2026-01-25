//! Match system.

use crate::config::GameConfig;
use crate::state::GameState;
use cm_core::world::World;

/// Match system.
pub struct MatchSystem;

impl MatchSystem {
    /// Run match day.
    pub fn run_match_day(&self, _cfg: &GameConfig, _world: &mut World, state: &mut GameState) {
        // Stub: simulate all matches for the day
        state.add_message("Match day! Check fixture results.");
        state.flags.match_day = false;
    }
}
