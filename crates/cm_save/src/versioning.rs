//! Save versioning.

/// Current save version.
pub const SAVE_VERSION: u32 = 1;

/// Check if a save version is compatible.
pub fn is_compatible(save_version: u32) -> bool {
    save_version <= SAVE_VERSION
}

/// Migrations for older save versions.
pub fn get_migrations(from_version: u32) -> Vec<&'static str> {
    let mut migrations = Vec::new();

    if from_version < 1 {
        migrations.push("v0_to_v1");
    }

    migrations
}
