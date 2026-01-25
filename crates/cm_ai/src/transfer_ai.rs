//! Transfer AI (stub).

use cm_core::economy::Money;
use cm_core::ids::{ClubId, PlayerId};
use cm_core::world::World;

/// Decide if should bid on player.
pub fn should_bid(_world: &World, _club_id: &ClubId, _player_id: &PlayerId) -> bool {
    // Stub: would check needs, budget, etc.
    false
}

/// Calculate bid amount.
pub fn calculate_bid(_world: &World, _player_id: &PlayerId, _budget: Money) -> Money {
    Money::from_major(1_000_000)
}
