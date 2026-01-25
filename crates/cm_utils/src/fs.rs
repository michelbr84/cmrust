//! Filesystem utilities.

use std::fs;
use std::path::Path;

use crate::errors::UtilError;

/// Read a file to string.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, UtilError> {
    Ok(fs::read_to_string(path)?)
}

/// Read a file to bytes.
pub fn read_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, UtilError> {
    Ok(fs::read(path)?)
}

/// Write string to file.
pub fn write_string<P: AsRef<Path>>(path: P, content: &str) -> Result<(), UtilError> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(fs::write(path, content)?)
}

/// Write bytes to file.
pub fn write_bytes<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<(), UtilError> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(fs::write(path, content)?)
}

/// Check if a path exists.
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Ensure directory exists.
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<(), UtilError> {
    Ok(fs::create_dir_all(path)?)
}
