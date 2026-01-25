//! Save format definition.

/// File extension for save files.
pub const SAVE_EXTENSION: &str = "cmsave";

/// Magic bytes at start of save file.
pub const MAGIC_BYTES: &[u8] = b"CMRS";

/// Save file format version.
pub const FORMAT_VERSION: u32 = 1;
