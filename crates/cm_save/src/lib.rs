//! # CM Save
//!
//! Save game system with compression and integrity verification.

pub mod compression;
pub mod errors;
pub mod format;
pub mod integrity;
pub mod snapshot;
pub mod versioning;

pub use errors::SaveError;
pub use snapshot::SaveSnapshot;
