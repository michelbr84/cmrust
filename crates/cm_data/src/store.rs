//! Data store abstraction.

use cm_core::world::World;
use crate::errors::DataError;

/// Abstract data store trait.
pub trait DataStore {
    /// Load the world.
    fn load_world(&self) -> Result<World, DataError>;

    /// Save the world.
    fn save_world(&self, world: &World) -> Result<(), DataError>;
}
