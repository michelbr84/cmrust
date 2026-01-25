//! Serde extension utilities.

use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

use crate::errors::UtilError;
use crate::fs;

/// Load JSON from file.
pub fn load_json<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, UtilError> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

/// Save as JSON to file.
pub fn save_json<T: Serialize, P: AsRef<Path>>(path: P, value: &T) -> Result<(), UtilError> {
    let content = serde_json::to_string_pretty(value)?;
    fs::write_string(path, &content)
}

/// Serialize to JSON string.
pub fn to_json<T: Serialize>(value: &T) -> Result<String, UtilError> {
    Ok(serde_json::to_string_pretty(value)?)
}

/// Deserialize from JSON string.
pub fn from_json<T: DeserializeOwned>(json: &str) -> Result<T, UtilError> {
    Ok(serde_json::from_str(json)?)
}
