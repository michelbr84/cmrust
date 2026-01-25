//! Players repository.

use cm_core::ids::{ClubId, PlayerId};
use cm_core::world::{Player, Position, World};

/// Get all players.
pub fn get_all(world: &World) -> Vec<&Player> {
    world.players.values().collect()
}

/// Get player by ID.
pub fn get_by_id<'a>(world: &'a World, id: &PlayerId) -> Option<&'a Player> {
    world.players.get(id)
}

/// Get players by club.
pub fn get_by_club<'a>(world: &'a World, club_id: &'a ClubId) -> Vec<&'a Player> {
    world
        .players
        .values()
        .filter(|p| p.club_id.as_ref() == Some(club_id))
        .collect()
}

/// Get players by position.
pub fn get_by_position(world: &World, position: Position) -> Vec<&Player> {
    world
        .players
        .values()
        .filter(|p| p.position == position)
        .collect()
}

/// Get free agents.
pub fn get_free_agents(world: &World) -> Vec<&Player> {
    world
        .players
        .values()
        .filter(|p| p.club_id.is_none())
        .collect()
}

/// Get players sorted by value.
pub fn get_by_value(world: &World) -> Vec<&Player> {
    let mut players: Vec<_> = world.players.values().collect();
    players.sort_by(|a, b| b.value.cmp(&a.value));
    players
}
