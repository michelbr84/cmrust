//! SQLite database connection.

use rusqlite::Connection;
use crate::errors::DataError;

/// SQLite database wrapper.
pub struct SqliteDb {
    conn: Connection,
}

impl SqliteDb {
    /// Open or create a database.
    pub fn open(path: &str) -> Result<Self, DataError> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    /// Open in-memory database.
    pub fn open_in_memory() -> Result<Self, DataError> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    /// Get connection reference.
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Run migrations.
    pub fn run_migrations(&self) -> Result<(), DataError> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS clubs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                nation_id TEXT,
                reputation INTEGER,
                data TEXT
            );

            CREATE TABLE IF NOT EXISTS players (
                id TEXT PRIMARY KEY,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                club_id TEXT,
                nation_id TEXT,
                position TEXT,
                data TEXT
            );

            CREATE TABLE IF NOT EXISTS saves (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at TEXT,
                data BLOB
            );
            "
        )?;
        Ok(())
    }
}
