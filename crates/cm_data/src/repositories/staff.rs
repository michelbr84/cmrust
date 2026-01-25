//! Staff repository (stub).

use cm_core::ids::StaffId;
use cm_core::world::{Staff, World};

/// Get all staff.
pub fn get_all(world: &World) -> Vec<&Staff> {
    world.staff.values().collect()
}

/// Get staff by ID.
pub fn get_by_id<'a>(world: &'a World, id: &StaffId) -> Option<&'a Staff> {
    world.staff.get(id)
}
