//! Squad building AI (stub).

use cm_core::ids::ClubId;
use cm_core::world::World;

/// Analyze squad needs.
pub fn analyze_squad_needs(_world: &World, _club_id: &ClubId) -> Vec<SquadNeed> {
    // Stub: would analyze positions, ages, quality
    Vec::new()
}

/// Squad need description.
pub struct SquadNeed {
    pub position: String,
    pub priority: u8,
    pub reason: String,
}
