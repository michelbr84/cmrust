//! Clubs repository.

use cm_core::ids::ClubId;
use cm_core::world::{Club, World};

/// Get all clubs.
pub fn get_all(world: &World) -> Vec<&Club> {
    world.clubs.values().collect()
}

/// Get club by ID.
pub fn get_by_id<'a>(world: &'a World, id: &ClubId) -> Option<&'a Club> {
    world.clubs.get(id)
}

/// Get clubs by nation.
pub fn get_by_nation<'a>(world: &'a World, nation_id: &str) -> Vec<&'a Club> {
    world
        .clubs
        .values()
        .filter(|c| c.nation_id.as_str() == nation_id)
        .collect()
}

/// Get clubs sorted by reputation.
pub fn get_by_reputation(world: &World) -> Vec<&Club> {
    let mut clubs: Vec<_> = world.clubs.values().collect();
    clubs.sort_by(|a, b| b.reputation.cmp(&a.reputation));
    clubs
}
