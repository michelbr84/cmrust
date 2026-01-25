//! Saves repository (stub).

use crate::db::SqliteDb;
use crate::errors::DataError;

/// Save metadata.
pub struct SaveSlot {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

/// List all save slots.
pub fn list_saves(_db: &SqliteDb) -> Result<Vec<SaveSlot>, DataError> {
    // Stub implementation
    Ok(Vec::new())
}

/// Delete a save slot.
pub fn delete_save(_db: &SqliteDb, _id: &str) -> Result<(), DataError> {
    // Stub implementation
    Ok(())
}
