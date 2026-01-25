//! Competitions repository.

use cm_core::ids::CompetitionId;
use cm_core::world::{Competition, CompetitionType, World};

/// Get all competitions.
pub fn get_all(world: &World) -> Vec<&Competition> {
    world.competitions.values().collect()
}

/// Get competition by ID.
pub fn get_by_id<'a>(world: &'a World, id: &CompetitionId) -> Option<&'a Competition> {
    world.competitions.get(id)
}

/// Get leagues.
pub fn get_leagues(world: &World) -> Vec<&Competition> {
    world
        .competitions
        .values()
        .filter(|c| c.competition_type == CompetitionType::League)
        .collect()
}

/// Get cups.
pub fn get_cups(world: &World) -> Vec<&Competition> {
    world
        .competitions
        .values()
        .filter(|c| c.competition_type == CompetitionType::Cup)
        .collect()
}
