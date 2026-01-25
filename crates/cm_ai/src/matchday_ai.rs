//! Matchday AI (stub).

use cm_core::ids::ClubId;
use cm_core::world::{Tactics, World};

/// Select starting lineup.
pub fn select_lineup(_world: &World, _club_id: &ClubId) -> Vec<String> {
    // Stub: would pick best 11
    Vec::new()
}

/// Select tactics for match.
pub fn select_tactics(_world: &World, _club_id: &ClubId, _opponent_id: &ClubId) -> Tactics {
    Tactics::default()
}
