//! Save system.

use crate::state::GameState;

/// Save system.
pub struct SaveSystem;

impl SaveSystem {
    /// Mark state as dirty (needs save).
    pub fn mark_dirty(&self, state: &mut GameState) {
        state.flags.dirty = true;
    }
}
