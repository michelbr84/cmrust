//! Referee entity.

use serde::{Deserialize, Serialize};
use crate::ids::{NationId, RefereeId};

/// A match referee.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Referee {
    pub id: RefereeId,
    pub name: String,
    pub nationality: NationId,
    pub strictness: u8,
    pub experience: u8,
}

impl Referee {
    /// Create a new referee.
    pub fn new(id: impl Into<RefereeId>, name: impl Into<String>, nationality: NationId) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            nationality,
            strictness: 50,
            experience: 50,
        }
    }
}
